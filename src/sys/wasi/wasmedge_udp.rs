use std::io;
use std::net::{self, SocketAddr};
use std::os::wasi::io::{FromRawFd, IntoRawFd};

use wasmedge_wasi_socket::socket;

pub fn bind(addr: SocketAddr) -> io::Result<net::UdpSocket> {
    let domain = match addr {
        SocketAddr::V4(_) => socket::AddressFamily::Inet4,
        SocketAddr::V6(_) => socket::AddressFamily::Inet6,
    };

    let s = socket::Socket::new(domain, socket::SocketType::Datagram)?;
    s.set_nonblocking(true)?;
    s.bind(&addr)?;

    let socket = unsafe { net::UdpSocket::from_raw_fd(s.into_raw_fd()) };

    Ok(socket)
}
