use std::io;
use std::net::Shutdown;
use std::pin::Pin;
use std::task::{Context, Poll};

use libc::{c_int, c_ushort, sa_family_t};
use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use tokio::io::unix::AsyncFd;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

// <bluetooth/hci.h>
const BTPROTO_HCI: c_int = 1;
const HCI_DEV_NONE: c_ushort = 0xffff;
const HCI_CHANNEL_CONTROL: c_ushort = 3;

#[repr(C)]
#[allow(non_camel_case_types)]
struct sockaddr_hci {
    hci_family: sa_family_t,
    hci_dev: c_ushort,
    hci_channel: c_ushort,
}

fn mgmt_open_bind() -> io::Result<Socket> {
    let domain = Domain::from(libc::AF_BLUETOOTH);
    let r#type = Type::RAW.nonblocking().cloexec();
    let proto = Protocol::from(BTPROTO_HCI);
    let sock = Socket::new(domain, r#type, Some(proto))?;

    let (_, addr) = unsafe {
        SockAddr::init(move |addr, _| {
            let addr = &mut *(addr as *mut sockaddr_hci);
            *addr = sockaddr_hci {
                hci_family: libc::AF_BLUETOOTH as sa_family_t,
                hci_dev: HCI_DEV_NONE,
                hci_channel: HCI_CHANNEL_CONTROL,
            };
            Ok(())
        })
    }?;
    sock.bind(&addr)?;
    Ok(sock)
}

#[derive(Debug)]
pub(crate) struct MgmtSocket {
    inner: AsyncFd<Socket>,
}

impl MgmtSocket {
    pub(crate) fn new() -> io::Result<Self> {
        let sock = mgmt_open_bind()?;
        let sock = AsyncFd::new(sock)?;
        Ok(Self { inner: sock })
    }
}

impl AsyncRead for MgmtSocket {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        loop {
            let mut guard = match self.inner.poll_read_ready(cx)? {
                Poll::Ready(guard) => guard,
                Poll::Pending => return Poll::Pending,
            };
            let result = guard.try_io(|fd| fd.get_ref().recv(unsafe { buf.unfilled_mut() }));
            match result {
                Ok(Ok(0)) => {}
                Ok(Ok(n)) => {
                    unsafe {
                        buf.assume_init(n);
                    }
                    buf.advance(n);
                    return Poll::Ready(Ok(()));
                }
                Ok(Err(err)) => return Poll::Ready(Err(err)),
                Err(..) => {}
            }
        }
    }
}

impl AsyncWrite for MgmtSocket {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        loop {
            let mut guard = match self.inner.poll_write_ready(cx)? {
                Poll::Ready(guard) => guard,
                Poll::Pending => return Poll::Pending,
            };
            let result = guard.try_io(|fd| fd.get_ref().send(buf));
            match result {
                Ok(Ok(0)) => {
                    return Poll::Ready(Err(io::Error::new(
                        io::ErrorKind::WriteZero,
                        "write zero.",
                    )))
                }
                Ok(Ok(n)) => return Poll::Ready(Ok(n)),
                Ok(Err(err)) => return Poll::Ready(Err(err)),
                Err(..) => {}
            }
        }
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        // Probably not effective..
        // https://github.com/tokio-rs/tokio/issues/1679
        // https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=6b6a158f01755843de85cdfdda8700a6
        let this = self.get_mut();
        this.inner.get_ref().shutdown(Shutdown::Write)?;
        Poll::Ready(Ok(()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::net::UnixDatagram;
    use std::thread;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn test_sock() {
        const N: usize = 1024;

        let (local, peer) = UnixDatagram::pair().unwrap();
        let t1 = thread::spawn(move || {
            for _ in 0..N {
                let mut buf = [0; b"Hello, World!".len()];
                let n = peer.recv(&mut buf[..]).unwrap();
                assert_eq!(n, buf.len());
                assert_eq!(&buf[..], b"Hello, World!");

                let n = peer.send(&b"Hello, World!"[..]).unwrap();
                assert_eq!(n, b"Hello, World!".len());
            }
            peer.shutdown(Shutdown::Both).unwrap();
        });

        local.set_nonblocking(true).unwrap();
        let local = Socket::from(local);
        let sock = MgmtSocket {
            inner: AsyncFd::new(local).unwrap(),
        };
        let (mut rx, mut tx) = tokio::io::split(sock);

        let t2 = tokio::spawn(async move {
            for _ in 0..N {
                tx.write_all(b"Hello, World!").await.unwrap();
            }
            tx.shutdown().await.unwrap();
        });

        let t3 = tokio::spawn(async move {
            for _ in 0..N {
                let mut buf = [0; b"Hello, World!".len()];
                let n = rx.read(&mut buf).await.unwrap();
                assert_eq!(n, b"Hello, World!".len());
                assert_eq!(&buf[..], b"Hello, World!");
            }
        });

        tokio::try_join!(t2, t3).unwrap();
        t1.join().unwrap();
    }
}
