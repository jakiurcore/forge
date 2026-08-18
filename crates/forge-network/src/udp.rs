//! Synchronous UDP sender/receiver helpers.

use crate::error::NetworkError;
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use std::time::Duration;

/// Bind a UDP socket and return it with the concrete local address.
pub fn bind_udp<A: ToSocketAddrs>(addr: A) -> Result<(UdpSocket, SocketAddr), NetworkError> {
    let socket = UdpSocket::bind(addr)?;
    let local = socket.local_addr()?;
    Ok((socket, local))
}

/// Set a receive timeout on a UDP socket.
pub fn set_recv_timeout(socket: &UdpSocket, timeout: Duration) -> Result<(), NetworkError> {
    socket.set_read_timeout(Some(timeout))?;
    Ok(())
}

/// Receive a single datagram, returning the data and sender address.
pub fn recv_from(socket: &UdpSocket, buf: &mut [u8]) -> Result<(usize, SocketAddr), NetworkError> {
    Ok(socket.recv_from(buf)?)
}

/// Send a datagram to `dest`.
pub fn send_to<A: ToSocketAddrs>(
    socket: &UdpSocket,
    buf: &[u8],
    dest: A,
) -> Result<usize, NetworkError> {
    Ok(socket.send_to(buf, dest)?)
}

/// A minimal echo loop for UDP: receive one datagram and send it back.
pub fn echo_once(socket: &UdpSocket) -> Result<(), NetworkError> {
    let mut buf = vec![0u8; 65535];
    let (n, peer) = recv_from(socket, &mut buf)?;
    send_to(socket, &buf[..n], peer)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn udp_echo_roundtrip() {
        let (server, server_local) = bind_udp("127.0.0.1:0").unwrap();
        let (client, _client_local) = bind_udp("127.0.0.1:0").unwrap();

        client.send_to(b"ping", server_local).unwrap();

        let mut buf = [0u8; 64];
        let (n, peer) = server.recv_from(&mut buf).unwrap();
        assert_eq!(&buf[..n], b"ping");

        server.send_to(&buf[..n], peer).unwrap();

        let mut resp = [0u8; 64];
        let (m, _) = client.recv_from(&mut resp).unwrap();
        assert_eq!(&resp[..m], b"ping");
    }
}
