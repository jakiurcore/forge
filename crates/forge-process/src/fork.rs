//! Controlled `fork()` experiments.
//!
//! These helpers are Linux/Unix-specific and use the `nix` crate.

use nix::unistd::{fork, getpid, getppid, ForkResult};

/// Outcome of a controlled fork.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForkOutcome {
    /// We are the parent; this is the child's PID.
    Parent {
        /// PID of the newly created child.
        child_pid: i32,
    },
    /// We are the child.
    Child,
}

/// Fork the current process and report the relationship.
///
/// Returns `Ok(ForkOutcome)` on success. This is intended for demonstrations
/// and experiments, not production process spawning (prefer `std::process::Command`).
pub fn demo_fork() -> Result<ForkOutcome, nix::Error> {
    match unsafe { fork() }? {
        ForkResult::Parent { child } => Ok(ForkOutcome::Parent {
            child_pid: child.as_raw(),
        }),
        ForkResult::Child => Ok(ForkOutcome::Child),
    }
}

/// Return the current PID and PPID as a tuple.
pub fn current_pids() -> (i32, i32) {
    (getpid().as_raw(), getppid().as_raw())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_pids_are_nonzero() {
        let (pid, ppid) = current_pids();
        assert!(pid > 0);
        assert!(ppid > 0);
    }
}
