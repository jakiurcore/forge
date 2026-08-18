//! Error types for concurrency primitives.

use std::fmt;

/// Errors that can occur when using concurrency primitives.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConcurrencyError {
    /// The work queue is full and the send would block.
    QueueFull,
    /// The thread pool or queue has been shut down.
    Shutdown,
    /// A generic error message.
    Other(String),
}

impl fmt::Display for ConcurrencyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConcurrencyError::QueueFull => write!(f, "work queue is full"),
            ConcurrencyError::Shutdown => write!(f, "concurrency primitive has shut down"),
            ConcurrencyError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ConcurrencyError {}
