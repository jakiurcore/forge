//! Error types for memory experiments and inspection.

use std::fmt;

/// Errors that can occur in memory modules.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemoryError {
    /// The requested address is not mapped.
    InvalidAddress(u64),
    /// Alignment requirement was not satisfied.
    Misaligned,
    /// Allocation failed because no suitable block was available.
    OutOfMemory,
    /// A system call failed.
    SystemError(String),
    /// A generic error message.
    Other(String),
}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryError::InvalidAddress(addr) => write!(f, "invalid address: 0x{:x}", addr),
            MemoryError::Misaligned => write!(f, "address is misaligned"),
            MemoryError::OutOfMemory => write!(f, "out of memory"),
            MemoryError::SystemError(msg) => write!(f, "system error: {}", msg),
            MemoryError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for MemoryError {}
