//! Helpers for `wait()` / `waitpid()` and exit-status decoding.

use crate::error::ProcessError;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::Pid;

/// Decoded child exit status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExitSummary {
    /// Exited normally with this code.
    Exited(u8),
    /// Killed by this signal.
    Signaled(String, i32),
    /// Stopped by this signal.
    Stopped(String, i32),
    /// Continued.
    Continued,
    /// Still running.
    Running,
    /// Unknown wait status.
    Unknown,
}

impl ExitSummary {
    /// Human-readable description.
    pub fn describe(&self) -> String {
        match self {
            ExitSummary::Exited(code) => format!("exited with status {}", code),
            ExitSummary::Signaled(sig, num) => format!("killed by signal {} ({})", sig, num),
            ExitSummary::Stopped(sig, num) => format!("stopped by signal {} ({})", sig, num),
            ExitSummary::Continued => "continued".to_string(),
            ExitSummary::Running => "still running".to_string(),
            ExitSummary::Unknown => "unknown status".to_string(),
        }
    }
}

/// Wait for a specific child PID and decode its status.
pub fn wait_for_child(pid: i32) -> Result<ExitSummary, ProcessError> {
    let status =
        waitpid(Some(Pid::from_raw(pid)), None).map_err(|e| ProcessError::SystemError(e.into()))?;
    Ok(decode_wait_status(status))
}

/// Decode a `nix` wait status.
pub fn decode_wait_status(status: WaitStatus) -> ExitSummary {
    match status {
        WaitStatus::Exited(_pid, code) => ExitSummary::Exited(code as u8),
        WaitStatus::Signaled(_pid, sig, _core_dump) => {
            ExitSummary::Signaled(format!("{:?}", sig), sig as i32)
        }
        WaitStatus::Stopped(_pid, sig) => ExitSummary::Stopped(format!("{:?}", sig), sig as i32),
        WaitStatus::Continued(_pid) => ExitSummary::Continued,
        WaitStatus::StillAlive => ExitSummary::Running,
        _ => ExitSummary::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_exited_status() {
        let status = WaitStatus::Exited(Pid::from_raw(1), 42);
        assert_eq!(decode_wait_status(status), ExitSummary::Exited(42));
    }
}
