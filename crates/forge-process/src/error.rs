//! Error types for process inspection and experimentation.

use std::fmt;
use std::io;

/// Errors that can occur when inspecting or experimenting with processes.
#[derive(Debug)]
pub enum ProcessError {
    /// The requested PID does not exist or `/proc/<pid>` is missing.
    ProcessNotFound(u32),
    /// The caller lacks permission to read the process information.
    PermissionDenied(u32),
    /// A `/proc` file could not be parsed.
    ParseError {
        /// File that failed to parse.
        file: String,
        /// Underlying error message.
        message: String,
    },
    /// A system call failed.
    SystemError(io::Error),
    /// A generic error with a message.
    Other(String),
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessError::ProcessNotFound(pid) => write!(f, "process {} not found", pid),
            ProcessError::PermissionDenied(pid) => {
                write!(f, "permission denied inspecting process {}", pid)
            }
            ProcessError::ParseError { file, message } => {
                write!(f, "failed to parse {}: {}", file, message)
            }
            ProcessError::SystemError(e) => write!(f, "system error: {}", e),
            ProcessError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ProcessError {}

impl From<io::Error> for ProcessError {
    fn from(err: io::Error) -> Self {
        ProcessError::SystemError(err)
    }
}

/// Convert an IO error for a specific PID into the most appropriate `ProcessError`.
pub fn io_error_for_pid(pid: u32, err: io::Error) -> ProcessError {
    match err.kind() {
        io::ErrorKind::NotFound => ProcessError::ProcessNotFound(pid),
        io::ErrorKind::PermissionDenied => ProcessError::PermissionDenied(pid),
        _ => ProcessError::SystemError(err),
    }
}
