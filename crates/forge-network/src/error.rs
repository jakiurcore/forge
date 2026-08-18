//! Error types for networking primitives and experiments.

use std::fmt;
use std::io;

/// Errors that can occur in networking modules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NetworkError {
    /// An I/O operation failed.
    Io(String),
    /// A frame exceeds the maximum allowed size.
    FrameTooLarge {
        /// The received or requested size.
        size: usize,
        /// The configured maximum.
        max: usize,
    },
    /// A frame header contained an invalid length.
    InvalidFrameLength,
    /// The message payload is too large for the protocol.
    PayloadTooLarge,
    /// An unknown or unsupported command was received.
    InvalidCommand(u8),
    /// A protocol status or response was malformed.
    InvalidResponse,
    /// A network operation timed out.
    Timeout,
    /// A generic error message.
    Other(String),
}

impl fmt::Display for NetworkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkError::Io(msg) => write!(f, "io error: {}", msg),
            NetworkError::FrameTooLarge { size, max } => {
                write!(f, "frame too large: {} (max {})", size, max)
            }
            NetworkError::InvalidFrameLength => write!(f, "invalid frame length"),
            NetworkError::PayloadTooLarge => write!(f, "payload too large"),
            NetworkError::InvalidCommand(cmd) => write!(f, "invalid command: {}", cmd),
            NetworkError::InvalidResponse => write!(f, "invalid response"),
            NetworkError::Timeout => write!(f, "operation timed out"),
            NetworkError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for NetworkError {}

impl From<io::Error> for NetworkError {
    fn from(err: io::Error) -> Self {
        NetworkError::Io(err.to_string())
    }
}

impl From<NetworkError> for io::Error {
    fn from(err: NetworkError) -> Self {
        match err {
            NetworkError::Timeout => io::Error::new(io::ErrorKind::TimedOut, err),
            _ => io::Error::new(io::ErrorKind::Other, err),
        }
    }
}
