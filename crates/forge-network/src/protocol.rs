//! Forge Protocol v1 — a small application protocol on top of TCP.
//!
//! Wire format:
//!
//! ```text
//! Request:  [length: u32 BE][command: u8][payload: bytes]
//! Response: [length: u32 BE][status: u8][payload: bytes]
//! ```
//!
//! `length` covers the command/status byte plus the payload bytes.
//!
//! Commands:
//! - `0x01` Ping
//! - `0x02` Echo
//! - `0x03` Status
//!
//! Status codes:
//! - `0x00` OK
//! - `0x01` BadRequest
//! - `0x02` Error

use crate::error::NetworkError;
use crate::framing::FrameCodec;

/// Maximum allowed payload size within a protocol message.
pub const MAX_PAYLOAD_SIZE: usize = 1024 * 1024;

/// Protocol commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Command {
    /// Request a pong response.
    Ping = 0x01,
    /// Request the server to echo the payload.
    Echo = 0x02,
    /// Request server status.
    Status = 0x03,
}

impl Command {
    /// Convert a raw byte into a `Command`.
    pub fn from_u8(value: u8) -> Result<Self, NetworkError> {
        match value {
            0x01 => Ok(Command::Ping),
            0x02 => Ok(Command::Echo),
            0x03 => Ok(Command::Status),
            other => Err(NetworkError::InvalidCommand(other)),
        }
    }

    /// Convert the command to its wire byte.
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

/// Response status codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Status {
    /// Request succeeded.
    Ok = 0x00,
    /// Request was malformed or unsupported.
    BadRequest = 0x01,
    /// Server-side error.
    Error = 0x02,
}

impl Status {
    /// Convert a raw byte into a `Status`.
    pub fn from_u8(value: u8) -> Result<Self, NetworkError> {
        match value {
            0x00 => Ok(Status::Ok),
            0x01 => Ok(Status::BadRequest),
            0x02 => Ok(Status::Error),
            _other => Err(NetworkError::InvalidResponse),
        }
    }

    /// Convert the status to its wire byte.
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

/// A Forge protocol request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    /// Command being requested.
    pub command: Command,
    /// Command-specific payload.
    pub payload: Vec<u8>,
}

impl Request {
    /// Create a new request, validating payload size.
    pub fn new(command: Command, payload: Vec<u8>) -> Result<Self, NetworkError> {
        if payload.len() > MAX_PAYLOAD_SIZE {
            return Err(NetworkError::PayloadTooLarge);
        }
        Ok(Request { command, payload })
    }

    /// Encode the request into a frame.
    pub fn encode(&self) -> Result<Vec<u8>, NetworkError> {
        let mut body = Vec::with_capacity(1 + self.payload.len());
        body.push(self.command.as_u8());
        body.extend_from_slice(&self.payload);
        FrameCodec::new().encode(&body)
    }

    /// Decode a request from a raw payload (the inside of a frame).
    pub fn decode(payload: &[u8]) -> Result<Self, NetworkError> {
        if payload.is_empty() {
            return Err(NetworkError::InvalidCommand(0));
        }
        let command = Command::from_u8(payload[0])?;
        let body = payload[1..].to_vec();
        if body.len() > MAX_PAYLOAD_SIZE {
            return Err(NetworkError::PayloadTooLarge);
        }
        Ok(Request {
            command,
            payload: body,
        })
    }
}

/// A Forge protocol response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    /// Response status.
    pub status: Status,
    /// Response payload.
    pub payload: Vec<u8>,
}

impl Response {
    /// Create a new response.
    pub fn new(status: Status, payload: Vec<u8>) -> Result<Self, NetworkError> {
        if payload.len() > MAX_PAYLOAD_SIZE {
            return Err(NetworkError::PayloadTooLarge);
        }
        Ok(Response { status, payload })
    }

    /// Encode the response into a frame.
    pub fn encode(&self) -> Result<Vec<u8>, NetworkError> {
        let mut body = Vec::with_capacity(1 + self.payload.len());
        body.push(self.status.as_u8());
        body.extend_from_slice(&self.payload);
        FrameCodec::new().encode(&body)
    }

    /// Decode a response from a raw payload (the inside of a frame).
    pub fn decode(payload: &[u8]) -> Result<Self, NetworkError> {
        if payload.is_empty() {
            return Err(NetworkError::InvalidResponse);
        }
        let status = Status::from_u8(payload[0])?;
        let body = payload[1..].to_vec();
        if body.len() > MAX_PAYLOAD_SIZE {
            return Err(NetworkError::PayloadTooLarge);
        }
        Ok(Response {
            status,
            payload: body,
        })
    }
}

/// Read a full request from a stream using the frame codec.
pub fn read_request(
    stream: &mut std::net::TcpStream,
    buffer: &mut Vec<u8>,
) -> Result<Request, NetworkError> {
    let codec = FrameCodec::new();
    loop {
        if let Some(frame) = codec.decode(buffer)? {
            return Request::decode(&frame);
        }
        let mut chunk = [0u8; 4096];
        match stream.read(&mut chunk) {
            Ok(0) => return Err(NetworkError::Io("connection closed".to_string())),
            Ok(n) => buffer.extend_from_slice(&chunk[..n]),
            Err(e) => return Err(NetworkError::Io(e.to_string())),
        }
    }
}

/// Write a response to a stream using the frame codec.
pub fn write_response(
    stream: &mut std::net::TcpStream,
    response: &Response,
) -> Result<(), NetworkError> {
    let bytes = response.encode()?;
    std::io::Write::write_all(stream, &bytes)?;
    Ok(())
}

use std::io::Read;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_encode_decode() {
        let req = Request::new(Command::Echo, b"hello".to_vec()).unwrap();
        let frame = req.encode().unwrap();
        let mut buffer = frame.clone();
        let codec = FrameCodec::new();
        let raw = codec.decode(&mut buffer).unwrap().unwrap();
        let decoded = Request::decode(&raw).unwrap();
        assert_eq!(req, decoded);
    }

    #[test]
    fn response_encode_decode() {
        let resp = Response::new(Status::Ok, b"pong".to_vec()).unwrap();
        let frame = resp.encode().unwrap();
        let mut buffer = frame.clone();
        let codec = FrameCodec::new();
        let raw = codec.decode(&mut buffer).unwrap().unwrap();
        let decoded = Response::decode(&raw).unwrap();
        assert_eq!(resp, decoded);
    }

    #[test]
    fn invalid_command() {
        assert!(matches!(
            Command::from_u8(0xFF),
            Err(NetworkError::InvalidCommand(0xFF))
        ));
    }

    #[test]
    fn empty_request_is_invalid() {
        assert!(matches!(
            Request::decode(&[]),
            Err(NetworkError::InvalidCommand(0))
        ));
    }

    #[test]
    fn oversized_payload() {
        let big = vec![0u8; MAX_PAYLOAD_SIZE + 1];
        assert!(matches!(
            Request::new(Command::Echo, big),
            Err(NetworkError::PayloadTooLarge)
        ));
    }
}
