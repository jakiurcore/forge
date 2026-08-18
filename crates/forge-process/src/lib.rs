//! Linux/Unix process inspection and experimentation for Forge.
//!
//! This crate provides reusable components for understanding the Unix process
//! model, including `/proc` parsing, file descriptor inspection, fork/exec
//! experiments, signal handling, and pipes.
//!
//! Much of the functionality is Linux-specific because it relies on `/proc` and
//! POSIX system calls. Platform-specific modules are clearly marked.

#![deny(missing_docs)]

pub mod error;
pub mod exec;
pub mod fds;
pub mod fork;
pub mod info;
pub mod inspector;
pub mod pipe;
pub mod signal;
pub mod wait;

/// Re-export commonly used types.
pub use error::{io_error_for_pid, ProcessError};
pub use fds::{list_fds, FdInfo, FdKind};
pub use fork::{current_pids, demo_fork, ForkOutcome};
pub use info::{parse_cmdline_file, parse_status_file, ProcessInfo, ProcessState};
pub use inspector::ProcessSnapshot;
pub use wait::{decode_wait_status, wait_for_child, ExitSummary};
