//! Helpers for resolving and working with socket addresses.

use crate::error::NetworkError;
use std::net::ToSocketAddrs;

/// Resolve a host:port string into a concrete socket address.
///
/// Returns the first available address or an error if resolution fails.
pub fn resolve<A: ToSocketAddrs>(addr: A) -> Result<std::net::SocketAddr, std::io::Error> {
    addr.to_socket_addrs()?
        .next()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "empty address list"))
}

/// List local network interface names.
///
/// On Linux this reads `/sys/class/net`. On other platforms it currently
/// returns an empty list.
pub fn local_interfaces() -> Result<Vec<String>, NetworkError> {
    #[cfg(target_os = "linux")]
    {
        let mut names = Vec::new();
        for entry in std::fs::read_dir("/sys/class/net")? {
            let entry = entry?;
            if let Some(name) = entry.file_name().to_str() {
                names.push(name.to_string());
            }
        }
        names.sort();
        Ok(names)
    }
    #[cfg(not(target_os = "linux"))]
    {
        Ok(Vec::new())
    }
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
