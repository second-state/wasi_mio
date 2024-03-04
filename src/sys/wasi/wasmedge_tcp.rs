use std::convert::TryInto;
use std::io;
use std::net::SocketAddr;
use std::os::fd::{FromRawFd, IntoRawFd, RawFd};
use wasmedge_wasi_socket::socket;
use wasmedge_wasi_socket::{TcpListener, TcpStream};

pub(crate) fn new_for_addr(address: SocketAddr) -> io::Result<RawFd> {
    let domain = match address {
        SocketAddr::V4(_) => socket::AddressFamily::Inet4,
        SocketAddr::V6(_) => socket::AddressFamily::Inet6,
    };

    let s = socket::Socket::new(domain, socket::SocketType::Stream)?;
    s.set_nonblocking(true)?;
    Ok(s.into_raw_fd())
}

pub(crate) fn bind(socket: &TcpListener, addr: SocketAddr) -> io::Result<()> {
    socket.as_ref().bind(&addr)?;
    Ok(())
}

pub(crate) fn connect(socket: &TcpStream, addr: SocketAddr) -> io::Result<()> {
    match socket.as_ref().connect(&addr) {
        Err(err) if err.raw_os_error() != Some(libc::EINPROGRESS) => Err(err),
        _ => Ok(()),
    }
}

pub(crate) fn listen(socket: &TcpListener, backlog: u32) -> io::Result<()> {
    let backlog = backlog.try_into().unwrap_or(i32::max_value());
    socket.as_ref().listen(backlog)?;
    Ok(())
}

pub(crate) fn set_reuseaddr(socket: &TcpListener, reuseaddr: bool) -> io::Result<()> {
    let val: i32 = i32::from(reuseaddr);

    socket.as_ref().setsockopt(
        socket::SocketOptLevel::SolSocket,
        socket::SocketOptName::SoReuseaddr,
        val,
    )?;
    Ok(())
}

pub(crate) fn accept(listener: &TcpListener) -> io::Result<(std::net::TcpStream, SocketAddr)> {
    use std::os::fd::IntoRawFd;
    listener.accept(true).map(|(s, addr)| {
        (
            unsafe { std::net::TcpStream::from_raw_fd(s.into_raw_fd()) },
            addr,
        )
    })
}
