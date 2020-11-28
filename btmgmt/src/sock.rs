use std::future::Future;
use std::io;
use std::mem::{self, MaybeUninit};
use std::pin::Pin;
use std::ptr;
use std::task::{Context, Poll};

use libc::{c_int, c_ushort, sa_family_t, sockaddr, socklen_t};
use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use tokio::io::unix::AsyncFd;

macro_rules! ready {
    ($e:expr) => {
        match $e {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(e) => e,
        }
    };
}

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
    let r#type = Type::raw().non_blocking().cloexec();
    let proto = Protocol::from(BTPROTO_HCI);
    let sock = Socket::new(domain, r#type, Some(proto))?;

    let addr = sockaddr_hci {
        hci_family: libc::AF_BLUETOOTH as sa_family_t,
        hci_dev: HCI_DEV_NONE,
        hci_channel: HCI_CHANNEL_CONTROL,
    }
    .into();
    let addr = unsafe {
        SockAddr::from_raw_parts(
            &addr as *const _,
            mem::size_of::<sockaddr_hci>() as socklen_t,
        )
    };
    sock.bind(&addr)?;
    Ok(sock)
}

#[derive(Debug)]
pub(crate) struct Send<'a, 'b>(&'a AsyncFd<Socket>, &'b [u8]);

impl<'a, 'b> Future for Send<'a, 'b> {
    type Output = io::Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let Send(sock, buf) = self.get_mut();

        loop {
            let mut g = ready!(sock.poll_write_ready(cx))?;
            match sock.get_ref().send(buf) {
                Ok(r) => return Poll::Ready(Ok(r)),
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => g.clear_ready(),
                Err(e) => return Poll::Ready(Err(e)),
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct Recv<'a, 'b>(&'a AsyncFd<Socket>, &'b mut [u8]);

impl<'a, 'b> Future for Recv<'a, 'b> {
    type Output = io::Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let Recv(sock, buf) = self.get_mut();

        loop {
            let mut g = ready!(sock.poll_read_ready(cx))?;
            match sock.get_ref().recv(buf) {
                Ok(r) => return Poll::Ready(Ok(r)),
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => g.clear_ready(),
                Err(e) => return Poll::Ready(Err(e)),
            }
        }
    }
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

    pub(crate) fn send<'b>(&self, buf: &'b [u8]) -> Send<'_, 'b> {
        Send(&self.inner, buf)
    }

    pub(crate) fn recv<'b>(&self, buf: &'b mut [u8]) -> Recv<'_, 'b> {
        Recv(&self.inner, buf)
    }
}
