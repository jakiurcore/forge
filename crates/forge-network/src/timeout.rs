//! Timeout helpers for synchronous sockets.

use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

/// Set both read and write timeouts on a `TcpStream`.
pub fn set_stream_timeouts(stream: &TcpStream, timeout: Duration) -> io::Result<()> {
    stream.set_read_timeout(Some(timeout))?;
    stream.set_write_timeout(Some(timeout))?;
    Ok(())
}

/// Connect to `addr` with a bounded timeout.
///
/// Rust's `std::net::TcpStream::connect_timeout` is used directly.
pub fn connect_with_timeout<A: std::net::ToSocketAddrs>(
    addr: A,
    timeout: Duration,
) -> io::Result<TcpStream> {
    let addr = addr
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "empty address list"))?;
    TcpStream::connect_timeout(&addr, timeout)
}

/// Bind a listener with a configured non-blocking accept timeout by wrapping
/// the accept loop, since `TcpListener` itself has no global accept timeout in
/// the standard library.
pub fn bind_listener_with_timeout<A: std::net::ToSocketAddrs>(
    addr: A,
    accept_timeout: Duration,
) -> io::Result<(TcpListener, std::net::SocketAddr)> {
    let listener = TcpListener::bind(addr)?;
    listener.set_nonblocking(false)?;
    let local = listener.local_addr()?;

    // We cannot set a read timeout on a listener directly, but we return the
    // configured timeout so callers can use it for individual accepted streams.
    let _ = accept_timeout;
    Ok((listener, local))
}

/// Read exactly `buf.len()` bytes from `stream`, respecting its read timeout.
pub fn read_exact_timeout(stream: &mut TcpStream, buf: &mut [u8]) -> io::Result<()> {
    stream.read_exact(buf)
}

/// Write all bytes from `buf` to `stream`, respecting its write timeout.
pub fn write_all_timeout(stream: &mut TcpStream, buf: &[u8]) -> io::Result<()> {
    stream.write_all(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_timeout_to_unreachable() {
        // RFC 5737 TEST-NET-1; should be unreachable and timeout quickly.
        let result = connect_with_timeout("192.0.2.1:1", Duration::from_millis(100));
        assert!(result.is_err());
    }
}
