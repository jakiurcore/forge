//! Synchronous TCP client/server helpers.

use crate::error::NetworkError;
use crate::timeout::{connect_with_timeout, set_stream_timeouts};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::time::Duration;

/// Bind a TCP listener and return it with the concrete local address.
pub fn bind_and_listen<A: ToSocketAddrs>(
    addr: A,
) -> Result<(TcpListener, std::net::SocketAddr), NetworkError> {
    let listener = TcpListener::bind(addr)?;
    let local = listener.local_addr()?;
    Ok((listener, local))
}

/// Connect to `addr` with a bounded timeout and optional per-operation timeouts.
pub fn connect<A: ToSocketAddrs>(
    addr: A,
    connect_timeout: Duration,
    operation_timeout: Option<Duration>,
) -> Result<TcpStream, NetworkError> {
    let stream = connect_with_timeout(addr, connect_timeout)?;
    if let Some(t) = operation_timeout {
        set_stream_timeouts(&stream, t)?;
    }
    Ok(stream)
}

/// A simple echo handler: reads until EOF and writes back everything received.
///
/// This is intentionally minimal and educational. It respects the stream's
/// configured read/write timeouts.
pub fn echo_handler(mut stream: TcpStream) -> Result<(), NetworkError> {
    let mut buf = [0u8; 4096];
    loop {
        match stream.read(&mut buf) {
            Ok(0) => return Ok(()),
            Ok(n) => {
                stream.write_all(&buf[..n])?;
            }
            Err(e) => return Err(NetworkError::Io(e.to_string())),
        }
    }
}

/// Run a blocking echo server on `addr` until an accept error occurs.
pub fn run_echo_server<A: ToSocketAddrs>(addr: A) -> Result<(), NetworkError> {
    let (listener, local) = bind_and_listen(addr)?;
    println!("echo server listening on {}", local);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = echo_handler(stream) {
                    eprintln!("connection error: {}", e);
                }
            }
            Err(e) => return Err(NetworkError::Io(e.to_string())),
        }
    }
    Ok(())
}

/// Send `message` to `addr` and return the response read until EOF or timeout.
pub fn send_message<A: ToSocketAddrs>(
    addr: A,
    message: &[u8],
    connect_timeout: Duration,
    operation_timeout: Duration,
) -> Result<Vec<u8>, NetworkError> {
    let mut stream = connect(addr, connect_timeout, Some(operation_timeout))?;
    stream.write_all(message)?;

    // Signal that we are done writing so a simple echo server can close.
    let _ = stream.shutdown(std::net::Shutdown::Write);

    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn echo_server_roundtrip() {
        let (listener, local) = bind_and_listen("127.0.0.1:0").unwrap();

        thread::spawn(move || {
            let (stream, _) = listener.accept().unwrap();
            echo_handler(stream).unwrap();
        });

        let response = send_message(
            local,
            b"hello, forge",
            Duration::from_secs(2),
            Duration::from_secs(2),
        )
        .unwrap();
        assert_eq!(response, b"hello, forge");
    }
}
