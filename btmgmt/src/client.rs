//! mgmt API client.
use std::collections::HashMap;
use std::future::Future;
use std::io;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use bytes::{Bytes, BytesMut};
use tokio::stream::Stream;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot::{self, Sender};
use tokio::sync::Mutex;

use crate::pack::{Error as UnpackError, Pack, Unpack};
use crate::packet::command::{Command, CommandCode, CommandInternal};
use crate::packet::event::{CommandComplete, CommandStatus, Event};
use crate::packet::{ControllerIndex, ErrorCode};
use crate::sock::MgmtSocket;

/// Management API Client's Error.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("channel send error.")]
    ChannelSend,
    #[error("channel recieve error. {0:}")]
    ChannelRecv(#[from] oneshot::error::RecvError),
    #[error("recieved data error. {0:}")]
    RecievedData(#[from] UnpackError),
    #[error("error reply. {0:?}")]
    ErrorReply(ErrorCode),
    #[error("empty reply")]
    EmptyReply,
}

/// IO loop task Error.
#[derive(Debug, thiserror::Error)]
pub enum TaskError {
    #[error("channel send error.")]
    ChannelSend,
    #[error("channel recieve error. {0:}")]
    ChannelRecv(#[from] oneshot::error::RecvError),
    #[error("recieved data error. {0:}")]
    RecievedData(#[from] UnpackError),
    #[error("io error. {0:}")]
    Io(#[from] io::Error),
}

type Msg = (
    ControllerIndex,
    CommandCode,
    Bytes,
    Sender<(ErrorCode, Option<Bytes>)>,
);

type EventQueues = Vec<UnboundedSender<(ControllerIndex, Event)>>;

#[derive(Debug)]
struct Task {
    ingress_rx: UnboundedReceiver<Msg>,
    sock: MgmtSocket,
    event_queues: Arc<Mutex<EventQueues>>,
}

impl Task {
    fn new(
        ingress_rx: UnboundedReceiver<Msg>,
        sock: MgmtSocket,
        event_queues: Arc<Mutex<EventQueues>>,
    ) -> Self {
        Self {
            ingress_rx,
            sock,
            event_queues,
        }
    }

    async fn run(self) -> Result<(), TaskError> {
        const BUF_LEN: usize = 8 * 1024;

        let mut buf = BytesMut::new();
        buf.resize(BUF_LEN, 0);

        let mut egress = HashMap::<_, Sender<_>>::new();
        let Self {
            mut ingress_rx,
            sock,
            event_queues,
            ..
        } = self;

        loop {
            buf.resize(BUF_LEN, 0);
            tokio::select! {
                r = sock.recv(&mut buf) => {
                    let len = r?;

                    let remain = buf.split_off(len);
                    log::trace!("< {:X}", buf);
                    let (index, event) = Unpack::unpack(&mut buf.as_ref())?;
                    buf.unsplit(remain);

                    match event {
                        Event::CommandComplete(CommandComplete { opcode, status, data, .. }) => {
                            if let Some(sender) = egress.remove(&(index, opcode)) {
                                sender.send((status, Some(data))).map_err(|_| TaskError::ChannelSend)?;
                            }
                        }
                        Event::CommandStatus(CommandStatus { opcode, status, .. }) => {
                            if let Some(sender) = egress.remove(&(index, opcode)) {
                                sender.send((status, None)).map_err(|_| TaskError::ChannelSend)?;
                            }
                        }
                        e => {
                            log::debug!("<< {:?}", e);
                            let mut event_queues = event_queues.lock().await;
                            event_queues.retain(|q| {
                                q.send((index.clone(), e.clone())).is_ok()
                            });
                        }
                    }
                }
                maybe_msg = ingress_rx.recv() => {
                    if let Some((index, code, data, sender)) = maybe_msg {
                        log::trace!("> {:X}", &data);
                        sock.send(&data).await?;
                        egress.insert((index, code), sender);
                    } else {
                        break Ok(());
                    }
                }
            }
        }
    }
}

/// Management API Event Stream.
#[derive(Debug)]
pub struct Events {
    queue: UnboundedReceiver<(ControllerIndex, Event)>,
}

impl Stream for Events {
    type Item = (ControllerIndex, Event);
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.get_mut().queue).poll_next(cx)
    }
}

/// Management API Client
///
/// # Example
///
/// ```no_run
/// use btmgmt::Client;
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// let (client, ioloop) = Client::open().unwrap();
/// let handle = tokio::spawn(ioloop);
///
/// // Do staff
///
/// handle.await.unwrap().unwrap();
/// # }
/// ```
#[derive(Debug)]
pub struct Client {
    ingress_tx: UnboundedSender<Msg>,
    event_queues: Arc<Mutex<EventQueues>>,
}

impl Client {
    /// Construct Management API Client.
    pub fn open() -> io::Result<(Self, impl Future<Output = Result<(), TaskError>>)> {
        let sock = MgmtSocket::new()?;
        let (ingress_tx, ingress_rx) = mpsc::unbounded_channel();
        let event_queues = Arc::new(Mutex::new(vec![]));
        let task = Task::new(ingress_rx, sock, event_queues.clone());
        let io_loop = task.run();
        Ok((
            Self {
                ingress_tx,
                event_queues,
            },
            io_loop,
        ))
    }

    /// Get Management API Event Stream.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use btmgmt::Client;
    /// use tokio::stream::StreamExt;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// # let (client, ioloop) = Client::open().unwrap();
    /// # let handle = tokio::spawn(ioloop);
    /// let mut events = client.events().await;
    ///
    /// while let Some((index, event)) = events.next().await {
    ///     // Do staff.
    /// }
    /// # }
    /// ```
    pub async fn events(&self) -> Events {
        let (tx, rx) = mpsc::unbounded_channel();
        let queues = self.event_queues.clone();
        let mut queues = queues.lock().await;
        queues.push(tx);
        Events { queue: rx }
    }

    /// Call Management API Command.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use btmgmt::Client;
    /// use btmgmt::command as cmd;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// # let (client, ioloop) = Client::open().unwrap();
    /// # let handle = tokio::spawn(ioloop);
    /// // ...
    /// let reply = client.call(0, cmd::ReadManagementVersionInformation::new()).await.unwrap();
    /// // Do staff
    /// # }
    /// ```
    pub async fn call<I, P>(&self, index: I, command: P) -> Result<P::Reply, Error>
    where
        I: Into<ControllerIndex>,
        P: Command,
    {
        log::debug!(">> {:?}", command);
        let index = index.into();
        let command = CommandInternal::from((index.clone(), command));
        let mut buf = BytesMut::new();
        command.pack(&mut buf);

        let (tx, rx) = oneshot::channel();
        self.ingress_tx
            .send((index, P::CODE, buf.freeze(), tx))
            .map_err(|_| Error::ChannelSend)?;

        let (status, data) = rx.await?;
        let reply = match (status, data) {
            (status, Some(mut data)) if status.success() => Ok(P::Reply::unpack(&mut data)?),
            (status, None) if status.success() => Err(Error::EmptyReply),
            (status, _) => Err(Error::ErrorReply(status)),
        };
        log::debug!("<< {:?}", reply);
        reply
    }
}
