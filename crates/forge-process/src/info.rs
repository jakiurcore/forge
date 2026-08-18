//! Parse process information from `/proc`.
//!
//! This module is Linux-specific because it relies on the `/proc` virtual
//! filesystem.

use crate::error::{io_error_for_pid, ProcessError};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Process state as reported by `/proc/<pid>/stat`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    /// Running or runnable.
    Running,
    /// Sleeping in an interruptible wait.
    Sleeping,
    /// Waiting in uninterruptible disk sleep.
    Waiting,
    /// Zombie.
    Zombie,
    /// Stopped.
    Stopped,
    /// Tracing stop.
    TracingStop,
    /// Dead.
    Dead,
    /// Unknown state character.
    Unknown(char),
}

impl ProcessState {
    /// Parse the single-character state from `/proc/<pid>/stat`.
    pub fn from_char(c: char) -> Self {
        match c {
            'R' => ProcessState::Running,
            'S' => ProcessState::Sleeping,
            'D' => ProcessState::Waiting,
            'Z' => ProcessState::Zombie,
            'T' => ProcessState::Stopped,
            't' => ProcessState::TracingStop,
            'X' | 'x' => ProcessState::Dead,
            _ => ProcessState::Unknown(c),
        }
    }

    /// Human-readable label.
    pub fn label(&self) -> &'static str {
        match self {
            ProcessState::Running => "running",
            ProcessState::Sleeping => "sleeping",
            ProcessState::Waiting => "waiting",
            ProcessState::Zombie => "zombie",
            ProcessState::Stopped => "stopped",
            ProcessState::TracingStop => "tracing stop",
            ProcessState::Dead => "dead",
            ProcessState::Unknown(_) => "unknown",
        }
    }
}

/// High-level process information assembled from `/proc`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessInfo {
    /// Process ID.
    pub pid: u32,
    /// Parent process ID.
    pub ppid: u32,
    /// Process name.
    pub name: String,
    /// Current process state.
    pub state: ProcessState,
    /// Real user ID.
    pub uid: Option<u32>,
    /// Real group ID.
    pub gid: Option<u32>,
    /// Number of threads.
    pub threads: Option<u32>,
    /// Virtual memory size in bytes.
    pub vmsize: Option<u64>,
    /// Resident set size in bytes.
    pub rss: Option<u64>,
    /// Command line arguments.
    pub cmdline: Vec<String>,
}

impl ProcessInfo {
    /// Load information for a process by PID.
    pub fn from_pid(pid: u32) -> Result<Self, ProcessError> {
        let proc_dir = PathBuf::from(format!("/proc/{}", pid));
        if !proc_dir.exists() {
            return Err(ProcessError::ProcessNotFound(pid));
        }

        let status = parse_status_file(pid)?;
        let stat = parse_stat_file(pid)?;
        let cmdline = parse_cmdline_file(pid)?;

        Ok(ProcessInfo {
            pid,
            ppid: stat.ppid,
            name: stat.name,
            state: stat.state,
            uid: status
                .get("Uid")
                .and_then(|s| s.split_whitespace().next()?.parse().ok()),
            gid: status
                .get("Gid")
                .and_then(|s| s.split_whitespace().next()?.parse().ok()),
            threads: status.get("Threads").and_then(|s| s.trim().parse().ok()),
            vmsize: status.get("VmSize").and_then(|s| parse_kb(s)),
            rss: status.get("VmRSS").and_then(|s| parse_kb(s)),
            cmdline,
        })
    }
}

fn parse_kb(value: &str) -> Option<u64> {
    // Values look like "1234 kB".
    let mut parts = value.split_whitespace();
    let number: u64 = parts.next()?.parse().ok()?;
    let unit = parts.next().unwrap_or("kB");
    match unit {
        "B" => Some(number),
        "kB" => Some(number * 1024),
        "MB" => Some(number * 1024 * 1024),
        "GB" => Some(number * 1024 * 1024 * 1024),
        _ => Some(number * 1024),
    }
}

/// Parse `/proc/<pid>/status` into a key-value map.
pub fn parse_status_file(pid: u32) -> Result<HashMap<String, String>, ProcessError> {
    let path = format!("/proc/{}/status", pid);
    let content = read_proc_file(pid, &path)?;
    let mut map = HashMap::new();
    for line in content.lines() {
        if let Some((key, value)) = line.split_once(':') {
            map.insert(key.trim().to_string(), value.trim().to_string());
        }
    }
    Ok(map)
}

#[derive(Debug)]
pub(crate) struct StatFields {
    name: String,
    ppid: u32,
    state: ProcessState,
}

/// Parse `/proc/<pid>/stat`.
///
/// The comm field may contain spaces and parentheses, so a simple split is
/// unreliable. This parser finds the first `(` and last `)` to extract the
/// command name, then splits the remainder.
pub(crate) fn parse_stat_file(pid: u32) -> Result<StatFields, ProcessError> {
    let path = format!("/proc/{}/stat", pid);
    let content = read_proc_file(pid, &path)?;

    let start = content.find('(').ok_or_else(|| ProcessError::ParseError {
        file: path.clone(),
        message: "missing opening parenthesis in stat".to_string(),
    })?;
    let end = content.rfind(')').ok_or_else(|| ProcessError::ParseError {
        file: path.clone(),
        message: "missing closing parenthesis in stat".to_string(),
    })?;

    let name = content[start + 1..end].to_string();
    let remainder: Vec<&str> = content[end + 2..].split_whitespace().collect();

    // Field indices after comm are: state(0), ppid(1), ...
    let state_char = remainder
        .first()
        .and_then(|s| s.chars().next())
        .ok_or_else(|| ProcessError::ParseError {
            file: path.clone(),
            message: "missing state field".to_string(),
        })?;
    let ppid = remainder
        .get(1)
        .and_then(|s| s.parse().ok())
        .ok_or_else(|| ProcessError::ParseError {
            file: path.clone(),
            message: "missing ppid field".to_string(),
        })?;

    Ok(StatFields {
        name,
        ppid,
        state: ProcessState::from_char(state_char),
    })
}

/// Parse `/proc/<pid>/cmdline`, where arguments are separated by NUL bytes.
pub fn parse_cmdline_file(pid: u32) -> Result<Vec<String>, ProcessError> {
    let path = format!("/proc/{}/cmdline", pid);
    let content = read_proc_file(pid, &path)?;
    if content.is_empty() {
        return Ok(Vec::new());
    }
    let args: Vec<String> = content
        .split('\0')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    Ok(args)
}

fn read_proc_file(pid: u32, path: &str) -> Result<String, ProcessError> {
    fs::read_to_string(path).map_err(|e| io_error_for_pid(pid, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_state_from_char() {
        assert_eq!(ProcessState::from_char('R'), ProcessState::Running);
        assert_eq!(ProcessState::from_char('S'), ProcessState::Sleeping);
        assert_eq!(ProcessState::from_char('Z'), ProcessState::Zombie);
    }

    #[test]
    fn parse_kb_values() {
        assert_eq!(parse_kb("1024 kB"), Some(1024 * 1024));
        assert_eq!(parse_kb("1 MB"), Some(1024 * 1024));
    }

    #[test]
    fn parse_cmdline_with_nul() {
        let raw = "hello\0world\0";
        let args: Vec<String> = raw
            .split('\0')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        assert_eq!(args, vec!["hello", "world"]);
    }

    #[test]
    fn parse_stat_line_with_spaces() {
        let sample = "123 (my process name) S 456 789 ...";
        let start = sample.find('(').unwrap();
        let end = sample.rfind(')').unwrap();
        let name = sample[start + 1..end].to_string();
        let remainder: Vec<&str> = sample[end + 2..].split_whitespace().collect();
        assert_eq!(name, "my process name");
        assert_eq!(remainder[0], "S");
        assert_eq!(remainder[1], "456");
    }

    #[test]
    fn self_info_loads() {
        let info = ProcessInfo::from_pid(std::process::id()).unwrap();
        assert_eq!(info.pid, std::process::id());
        assert!(!info.name.is_empty());
    }

    #[test]
    fn missing_pid_errors() {
        let result = ProcessInfo::from_pid(999_999);
        assert!(matches!(
            result,
            Err(ProcessError::ProcessNotFound(999_999))
        ));
    }
}
