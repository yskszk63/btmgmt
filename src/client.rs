use std::mem::MaybeUninit;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
use std::future::Future;

use futures::{Sink, Stream, FutureExt, StreamExt, SinkExt};
use futures::lock::Mutex;
use futures::stream::{SplitSink, SplitStream};
use bytes::{Buf, BufMut, BytesMut};
use tokio::io::{self, AsyncRead, AsyncWrite, ReadBuf};
use tokio::sync::mpsc;

use crate::pack::{self, Unpack};
use crate::ControllerIndex;
use crate::event::{self, Events};
use crate::command::{self, Commands};
use crate::sock::MgmtSocket;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Pack(#[from] pack::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

struct EventStream<IO> {
    io: IO,
    rxbuf: BytesMut,
    txbuf: BytesMut,
}

impl<IO> EventStream<IO> {
    fn new(io: IO) -> Self {
        Self {
            io,
            rxbuf: BytesMut::new(),
            txbuf: BytesMut::new(),
        }
    }
}

impl<IO> Stream for EventStream<IO> where IO: AsyncRead + Unpin {
    type Item = Result<(ControllerIndex, Events)>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let Self { io, ref mut rxbuf, .. } = self.get_mut();

        loop {
            rxbuf.reserve(1024 * 8);

            let dst = rxbuf.chunk_mut();
            let dst = unsafe { &mut *(dst as *mut _ as *mut [MaybeUninit<u8>]) };
            let mut b = ReadBuf::uninit(dst);
            if Poll::Pending == Pin::new(io).poll_read(cx, &mut b)? {
                return Poll::Pending;
            };
            let n = b.filled().len();
            if n == 0 && !rxbuf.has_remaining() {
                return Poll::Ready(None);
            }
            drop(b);
            unsafe { rxbuf.advance_mut(n); }

            // TODO partial read
            let mut reader = rxbuf.reader();
            let (index, event) = event::unpack_events(&mut reader)?;
            *rxbuf = BytesMut::from(rxbuf.as_ref());
            return Poll::Ready(Some(Ok((index, event))));
        }
    }
}

impl<IO> Sink<(ControllerIndex, Commands)> for EventStream<IO> where IO: AsyncWrite + Unpin {
    type Error = Error;

    fn poll_ready(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, (index, commands): (ControllerIndex, Commands)) -> Result<()> {
        let Self { txbuf, .. } = self.get_mut();

        let mut write = txbuf.writer();
        command::pack_command(&index, &commands, &mut write)?;
        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        let this = self.get_mut();

        loop {
            if this.txbuf.has_remaining() {
                let n = match Pin::new(&mut this.io).poll_write(cx, &this.txbuf)? {
                    Poll::Ready(n) => n,
                    Poll::Pending => return Poll::Pending,
                };
                this.txbuf.advance(n);
            } else {
                if let Poll::Pending = Pin::new(&mut this.io).poll_flush(cx) {
                    return Poll::Pending;
                }
                return Poll::Ready(Ok(()));
            }
        }
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        let Self { io, .. } = self.get_mut();

        if let Poll::Pending = Pin::new(io).poll_shutdown(cx)? {
            return Poll::Pending;
        }

        Poll::Ready(Ok(()))
    }
}

struct RecvInner<S> {
    stream: S,
    wakers: Vec<Waker>,
    head: Option<Result<(ControllerIndex, Events)>>,
    subscribers: Vec<mpsc::UnboundedSender<(ControllerIndex, Events)>>,
}

struct Recv<S> {
    inner: Arc<Mutex<RecvInner<S>>>,
}

impl<S> Future for Recv<S> where S: Stream<Item = Result<(ControllerIndex, Events)>> + Unpin {
    type Output = Result<Option<(ControllerIndex, Events)>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            let mut inner = match self.inner.lock().poll_unpin(cx) {
                Poll::Ready(inner) => inner,
                Poll::Pending => return Poll::Pending,
            };

            if let Some(head) = inner.head.take() {
                return Poll::Ready(head.map(Some));
            }

            let result = match inner.stream.poll_next_unpin(cx) {
                Poll::Ready(result) => result,
                Poll::Pending => return Poll::Pending,
            };

            for w in inner.wakers.drain(..) {
                w.wake();
            }

            match result {
                result @ Some(Ok((_, Events::CommandComplete(..) | Events::CommandStatus(..))) | Err(..)) => inner.head = result,
                Some(Ok(events)) => {
                    for tx in &inner.subscribers {
                        tx.send(events.clone()).ok();
                    }
                }
                None => {
                    inner.subscribers.clear();
                    return Poll::Ready(Ok(None));
                }
            }
        }
    }
}

struct Next<S> {
    inner: Arc<Mutex<RecvInner<S>>>,
}

impl<S> Future for Next<S> where S: Stream<Item = Result<(ControllerIndex, Events)>> + Unpin {
    type Output = Option<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            let mut inner = match self.inner.lock().poll_unpin(cx) {
                Poll::Ready(inner) => inner,
                Poll::Pending => return Poll::Pending,
            };

            if inner.head.is_some() {
                inner.wakers.push(cx.waker().clone());
                return Poll::Pending;
            }

            let result = match inner.stream.poll_next_unpin(cx) {
                Poll::Ready(result) => result,
                Poll::Pending => return Poll::Pending,
            };

            for w in inner.wakers.drain(..) {
                w.wake();
            }

            match result {
                result @ Some(Ok((_, Events::CommandComplete(..) | Events::CommandStatus(..))) | Err(..)) => inner.head = result,
                Some(Ok(events)) => {
                    for tx in &inner.subscribers {
                        tx.send(events.clone()).ok();
                    }
                }
                None => {
                    inner.subscribers.clear();
                    return Poll::Ready(None);
                }
            }
        }
    }
}

struct Receive<S>(Arc<Mutex<RecvInner<S>>>);

impl<S> Clone for Receive<S> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<S> Receive<S> {
    fn new(stream: S) -> Self {
        Self(Arc::new(Mutex::new(RecvInner {
            stream,
            wakers: Default::default(),
            head: Default::default(),
            subscribers: vec![],
        })))
    }
}

impl<S> Receive<S> where S: Stream<Item = Result<(ControllerIndex, Events)>> + Unpin {
    fn recv(&self) -> Recv<S> {
        Recv {
            inner: self.0.clone(),
        }
    }

    fn next(&self) -> Next<S> {
        Next {
            inner: self.0.clone(),
        }
    }

    async fn subscribe(&self) -> mpsc::UnboundedReceiver<(ControllerIndex, Events)> {
        let (tx, rx) = mpsc::unbounded_channel();

        let mut inner = self.0.lock().await;
        inner.subscribers.push(tx);
        rx
    }
}

pub struct EventSubscribe {
    receive: Receive<SplitStream<EventStream<MgmtSocket>>>,
    rx: mpsc::UnboundedReceiver<(ControllerIndex, Events)>,
}

impl Stream for EventSubscribe {
    type Item = (ControllerIndex, Events);

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let Self { receive, rx } = self.get_mut();

        loop {
            match rx.poll_recv(cx) {
                Poll::Ready(result) => return Poll::Ready(result),
                Poll::Pending => {}
            }

            match receive.next().poll_unpin(cx) {
                Poll::Ready(..) => {}
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}

pub struct Client {
    rx:  Receive<SplitStream<EventStream<MgmtSocket>>>,
    tx: Arc<Mutex<SplitSink<EventStream<MgmtSocket>, (ControllerIndex, Commands)>>>,
}

impl Client {
    pub fn new() -> Result<Self> {
        let sock = MgmtSocket::new()?;
        let stream = EventStream::new(sock);
        let (tx, rx) = stream.split();
        Ok(Self {
            rx: Receive::new(rx),
            tx: Arc::new(Mutex::new(tx)),
        })
    }

    pub async fn events(&self) -> EventSubscribe {
        let rx = self.rx.subscribe().await;
        EventSubscribe {
            receive: Receive(self.rx.0.clone()),
            rx,
        }
    }

    pub fn call<C, I>(&self, index: I, command: C) -> impl Future<Output = Result<C::Reply>> + 'static where C: command::Command + 'static, I: Into<ControllerIndex> {
        let rx = self.rx.clone();
        let tx = self.tx.clone();

        Self::call_inner(index.into(), command, rx, tx)
    }

    async fn call_inner<C>(index: ControllerIndex,
        command: C, rx: Receive<SplitStream<EventStream<MgmtSocket>>>,
        tx: Arc<Mutex<SplitSink<EventStream<MgmtSocket>, (ControllerIndex, Commands)>>>) -> Result<C::Reply> where C: command::Command {
        let command = command.into();

        let mut tx = tx.lock().await;
        tx.send((index.clone(), command)).await?;

        let result = rx.recv().await?.unwrap(); // TODO EOF
        if index != result.0 {
            todo!()
        }
        match result.1 {
            Events::CommandComplete(comp) => {
                let mut data = &comp.data[..];
                let result = C::Reply::unpack(&mut data)?;
                Ok(result)
            }
            Events::CommandStatus(status) => {
                todo!()
            }
            _ => todo!(),
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::event::CommandComplete;
    use crate::packet::ErrorCode;
    use crate::command::CommandCode;
    use std::array::IntoIter;

    use super::*;
    use futures::{SinkExt, StreamExt};

    #[tokio::test]
    async fn test_stream_recv() {
        let packet = IntoIter::new([0x01u8, 0x00, 0xFF, 0xFF, 0x06, 0x00, 0x01, 0x00, 0x00, 0x01, 0x13, 0x00])
            .chain([0x01u8, 0x00, 0xFF, 0xFF, 0x06, 0x00, 0x01, 0x00, 0x00, 0x01, 0x13, 0x00])
            .chain([0x01u8, 0x00, 0xFF, 0xFF, 0x06, 0x00, 0x01, 0x00, 0x00, 0x01, 0x13, 0x00])
            .collect::<Vec<_>>();
        let mut stream = EventStream {
            io: &packet[..],
            rxbuf: BytesMut::with_capacity(32),
            txbuf: BytesMut::with_capacity(32),
        };

        let mut n = 0usize;
        while let Some(r) = stream.next().await {
            let (index, event) = r.unwrap();
            assert_eq!(ControllerIndex::NonController, index);
            if let Events::CommandComplete(CommandComplete { opcode, status, data } ) = event {
                assert_eq!(CommandCode::ReadManagementVersionInformation, opcode);
                assert_eq!(ErrorCode::Success, status);
                assert_eq!(&[0x01, 0x13, 0x00][..], data.as_ref());
            } else {
                panic!()
            };
            n += 1;
        }
        assert_eq!(3, n);
    }

    #[tokio::test]
    async fn test_stream_send() {
        let io = <Vec<u8>>::new();

        let mut stream = EventStream {
            io,
            rxbuf: BytesMut::default(),
            txbuf: BytesMut::default(),
        };

        let i = ControllerIndex::ControllerId(0);
        let c = command::SetPowered(true).into();
        stream.send((i, c)).await.unwrap();
    }
}
