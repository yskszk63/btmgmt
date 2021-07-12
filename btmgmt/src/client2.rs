use std::mem::MaybeUninit;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::Stream;
use bytes::{Buf, BufMut, BytesMut};
use tokio::io::{self, AsyncRead, ReadBuf};

use crate::pack::{self, Unpack};
use crate::ControllerIndex;
use crate::event::Event;

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
}

impl<IO> Stream for EventStream<IO> where IO: AsyncRead + Unpin {
    type Item = Result<(ControllerIndex, Event)>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let Self { io, rxbuf } = self.get_mut();

        loop {
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

            // TODO partial
            let (index, event) = Unpack::unpack(rxbuf)?;
            *rxbuf = BytesMut::from(rxbuf.as_ref());
            return Poll::Ready(Some(Ok((index, event))));
        }
    }
}

/*
impl<IO> Sink<Command> for EventStream<IO> where IO: AsyncWrite + Unpin {
}
*/

#[cfg(test)]
mod tests {
    use crate::event::CommandComplete;
    use crate::packet::ErrorCode;
    use crate::command::CommandCode;
    use std::array::IntoIter;

    use super::*;
    use futures::StreamExt;

    #[tokio::test]
    async fn test_recv() {
        let packet = IntoIter::new([0x01u8, 0x00, 0xFF, 0xFF, 0x06, 0x00, 0x01, 0x00, 0x00, 0x01, 0x13, 0x00])
            .chain([0x01u8, 0x00, 0xFF, 0xFF, 0x06, 0x00, 0x01, 0x00, 0x00, 0x01, 0x13, 0x00])
            .chain([0x01u8, 0x00, 0xFF, 0xFF, 0x06, 0x00, 0x01, 0x00, 0x00, 0x01, 0x13, 0x00])
            .collect::<Vec<_>>();
        let mut stream = EventStream {
            io: &packet[..],
            rxbuf: BytesMut::with_capacity(32),
        };

        let mut n = 0;
        while let Some(r) = stream.next().await {
            let (index, event) = r.unwrap();
            assert_eq!(ControllerIndex::NonController, index);
            if let Event::CommandComplete(CommandComplete { opcode, status, data } ) = event {
                assert_eq!(CommandCode::ReadManagementVersionInformation, opcode);
                assert_eq!(ErrorCode::Success, status);
                assert_eq!(&[0x01, 0x13, 0x00][..], data);
            } else {
                panic!()
            };
            n += 1;
        }
        assert_eq!(3, n);
    }
}
