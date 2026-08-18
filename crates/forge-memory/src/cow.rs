//! Educational copy-on-write experiment.
//!
//! This module forks a child process and observes that parent and child share
//! pages until one writes. It does not make unsupported claims about kernel
//! internals; it measures observable RSS behavior.

use crate::error::MemoryError;
use nix::unistd::{fork, ForkResult};
use std::process;
use std::time::Duration;

/// Allocate a large buffer, optionally touch it, fork, and report the PID of
/// the child. The caller can then observe `/proc/<pid>/smaps` or RSS.
///
/// This is a demonstration, not a precise measurement tool.
pub fn fork_and_touch(size: usize, touch_child: bool) -> Result<i32, MemoryError> {
    let mut buffer = vec![0u8; size];

    // Touch all pages in the parent.
    for i in (0..size).step_by(4096) {
        buffer[i] = 1;
    }

    match unsafe { fork() }.map_err(|e| MemoryError::SystemError(e.to_string()))? {
        ForkResult::Parent { child } => Ok(child.as_raw()),
        ForkResult::Child => {
            if touch_child {
                for i in (0..size).step_by(4096) {
                    buffer[i] = 2;
                }
            }
            // Sleep briefly so the parent can inspect us.
            std::thread::sleep(Duration::from_millis(200));
            process::exit(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fork_without_touch() {
        let child = fork_and_touch(1024 * 1024, false).unwrap();
        // Reap the child to avoid zombies.
        let _ = nix::sys::wait::waitpid(Some(nix::unistd::Pid::from_raw(child)), None);
        assert!(child > 0);
    }
}
