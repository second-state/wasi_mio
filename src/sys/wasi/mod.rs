//! # Notes
//!
//! The current implementation is somewhat limited. The `Waker` is not
//! implemented, as at the time of writing there is no way to support to wake-up
//! a thread from calling `poll_oneoff`.
//!
//! Furthermore the (re/de)register functions also don't work while concurrently
//! polling as both registering and polling requires a lock on the
//! `subscriptions`.
//!
//! Finally `Selector::try_clone`, required by `Registry::try_clone`, doesn't
//! work. However this could be implemented by use of an `Arc`.
//!
//! In summary, this only (barely) works using a single thread.

cfg_os_poll! {
    cfg_io_source!{
        mod poll;
        pub use poll::*;
    }
}

cfg_net! {
    #[cfg(not(wasmedge))]
    pub(crate) mod tcp {
        use std::io;
        use std::net::{self, SocketAddr};

        pub(crate) fn accept(listener: &net::TcpListener) -> io::Result<(net::TcpStream, SocketAddr)> {
            let (stream, addr) = listener.accept()?;
            stream.set_nonblocking(true)?;
            Ok((stream, addr))
        }
    }

    #[cfg_attr(wasmedge, path = "wasmedge_tcp.rs")]
    pub(crate) mod tcp;

    #[cfg_attr(wasmedge, path = "wasmedge_udp.rs")]
    pub(crate) mod udp;
}
