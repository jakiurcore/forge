//! Helpers for resolving and working with socket addresses.

use std::net::ToSocketAddrs;

/// Resolve a host:port string into a concrete socket address.
///
/// Returns the first available address or an error if resolution fails.
pub fn resolve<A: ToSocketAddrs>(addr: A) -> Result<std::net::SocketAddr, std::io::Error> {
    addr.to_socket_addrs()?
        .next()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "empty address list"))
}

/// Bind a TCP listener to an address, returning the bound address (including
/// the OS-assigned port when `0` was requested).
pub fn bind_listener<A: ToSocketAddrs>(
    addr: A,
) -> Result<(std::net::TcpListener, std::net::SocketAddr), std::io::Error> {
    let listener = std::net::TcpListener::bind(addr)?;
    let local = listener.local_addr()?;
    Ok((listener, local))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_localhost() {
        let addr = resolve("localhost:0").unwrap();
        assert!(addr.port() == 0);
        assert!(addr.ip().is_loopback());
    }

    #[test]
    fn bind_ephemeral_port() {
        let (listener, local) = bind_listener("127.0.0.1:0").unwrap();
        assert!(local.port() > 0);
        drop(listener);
    }
}
