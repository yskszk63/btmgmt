use std::io;
use std::mem::{self, MaybeUninit};
use std::ptr;

use libc::{c_int, c_ushort, sa_family_t, sockaddr, socklen_t};
use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use tokio::net::UdpSocket;

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
pub(crate) struct MgmtSocket {
    inner: UdpSocket,
}

impl MgmtSocket {
    pub(crate) fn new() -> io::Result<Self> {
        let sock = mgmt_create()?;
        let sock = sock.into_udp_socket();
        let sock = UdpSocket::from_std(sock)?;
        Ok(Self { inner: sock })
    }

    pub(crate) async fn send(&self, buf: &[u8]) -> io::Result<usize> {
        self.inner.send(buf).await
    }

    pub(crate) async fn recv(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.recv(buf).await
    }
}
