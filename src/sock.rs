use std::io;
use std::mem::{self, MaybeUninit};
use std::pin::Pin;
use std::ptr;
use std::task::{Context, Poll};

use libc::{c_int, c_ushort, sa_family_t, sockaddr, socklen_t};
use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use tokio::io::unix::AsyncFd;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

// <bluetooth/hci.h>
const BTPROTO_HCI: c_int = 1;
const HCI_DEV_NONE: c_ushort = 0xffff;
const HCI_CHANNEL_CONTROL: c_ushort = 3;

#[derive(Debug)]
#[repr(C)]
#[allow(non_camel_case_types)]
struct sockaddr_hci {
    hci_family: sa_family_t,
    hci_dev: c_ushort,
    hci_channel: c_ushort,
}

impl From<sockaddr_hci> for sockaddr {
    fn from(s: sockaddr_hci) -> Self {
        let mut result = MaybeUninit::<sockaddr>::uninit();
        unsafe {
            ptr::copy_nonoverlapping(
                &s as *const _ as *const u8,
                &mut result as *mut _ as *mut u8,
                mem::size_of::<sockaddr_hci>(),
            );
            result.assume_init()
        }
    }
}

fn mgmt_create() -> io::Result<Socket> {
    let domain = Domain::from(libc::AF_BLUETOOTH);
    let r#type = Type::RAW.nonblocking().cloexec();
    let proto = Protocol::from(BTPROTO_HCI);
    let sock = Socket::new(domain, r#type, Some(proto))?;

    let (_, addr) = unsafe {
        SockAddr::init(move |addr, len| {
            let addr = mem::transmute::<_, &mut sockaddr_hci>(addr);
            *addr = sockaddr_hci {
                hci_family: libc::AF_BLUETOOTH as sa_family_t,
                hci_dev: HCI_DEV_NONE,
                hci_channel: HCI_CHANNEL_CONTROL,
            };
            *len = mem::size_of::<sockaddr_hci>() as socklen_t;
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
        let sock = mgmt_create()?;
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
        let mut guard = match self.inner.poll_read_ready(cx)? {
            Poll::Ready(guard) => guard,
            Poll::Pending => return Poll::Pending,
        };
        let result = guard.try_io(|fd| fd.get_ref().recv(unsafe { buf.unfilled_mut() }));
        match result {
            Ok(Ok(n)) => {
                unsafe {
                    buf.assume_init(n);
                }
                buf.advance(n);
                Poll::Ready(Ok(()))
            }
            Ok(Err(err)) => Poll::Ready(Err(err)),
            Err(..) => Poll::Pending,
        }
    }
}

impl AsyncWrite for MgmtSocket {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let mut guard = match self.inner.poll_write_ready(cx)? {
            Poll::Ready(guard) => guard,
            Poll::Pending => return Poll::Pending,
        };
        let result = guard.try_io(|fd| fd.get_ref().send(buf));
        match result {
            Ok(Ok(0)) => Poll::Ready(Err(io::Error::new(io::ErrorKind::WriteZero, "write zero."))),
            Ok(Ok(n)) => Poll::Ready(Ok(n)),
            Ok(Err(err)) => Poll::Ready(Err(err)),
            Err(..) => Poll::Pending,
        }
    }
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }
    fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }
}
