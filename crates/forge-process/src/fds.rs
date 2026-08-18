//! Inspect open file descriptors of a process via `/proc/<pid>/fd`.
//!
//! Linux-specific.

use crate::error::{io_error_for_pid, ProcessError};
use std::fs;
use std::path::PathBuf;

/// Classification of an open file descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FdKind {
    /// Terminal or pseudoterminal.
    Terminal,
    /// Regular file.
    File,
    /// Pipe.
    Pipe,
    /// Socket.
    Socket,
    /// Anonymous inode (eventfd, timerfd, signalfd, epoll, inotify, etc.).
    AnonInode,
    /// Directory.
    Directory,
    /// Symbolic link to something else.
    Other,
    /// Target could not be resolved.
    Unknown,
}

impl FdKind {
    /// Classify a target path string.
    pub fn classify(target: &str) -> Self {
        if target.starts_with("/dev/pts/") || target == "/dev/tty" {
            FdKind::Terminal
        } else if target.starts_with("pipe:[") {
            FdKind::Pipe
        } else if target.starts_with("socket:[") {
            FdKind::Socket
        } else if target.starts_with("anon_inode:") {
            FdKind::AnonInode
        } else if target.starts_with('/') && target.ends_with('/') {
            FdKind::Directory
        } else if target.starts_with('/') {
            FdKind::File
        } else {
            FdKind::Other
        }
    }
}

/// Description of a single open file descriptor.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FdInfo {
    /// File descriptor number.
    pub fd: u32,
    /// Classified kind.
    pub kind: FdKind,
    /// Symlink target as reported by `/proc/<pid>/fd/<fd>`.
    pub target: String,
}

/// List open file descriptors for a process.
pub fn list_fds(pid: u32) -> Result<Vec<FdInfo>, ProcessError> {
    let fd_dir = PathBuf::from(format!("/proc/{}/fd", pid));
    if !fd_dir.exists() {
        return Err(ProcessError::ProcessNotFound(pid));
    }

    let mut entries = Vec::new();
    for entry in fs::read_dir(&fd_dir).map_err(|e| io_error_for_pid(pid, e))? {
        let entry = entry?;
        let file_name = entry.file_name();
        let fd_str = file_name.to_string_lossy();
        let fd: u32 = match fd_str.parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        let target = match fs::read_link(entry.path()) {
            Ok(path) => path.to_string_lossy().into_owned(),
            Err(_) => "?".to_string(),
        };

        entries.push(FdInfo {
            fd,
            kind: FdKind::classify(&target),
            target,
        });
    }

    entries.sort_by_key(|info| info.fd);
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_targets() {
        assert_eq!(FdKind::classify("/dev/pts/2"), FdKind::Terminal);
        assert_eq!(FdKind::classify("/dev/tty"), FdKind::Terminal);
        assert_eq!(FdKind::classify("pipe:[1234]"), FdKind::Pipe);
        assert_eq!(FdKind::classify("socket:[1234]"), FdKind::Socket);
        assert_eq!(FdKind::classify("anon_inode:[eventfd]"), FdKind::AnonInode);
        assert_eq!(FdKind::classify("/tmp/file.log"), FdKind::File);
    }

    #[test]
    fn list_self_fds() {
        let fds = list_fds(std::process::id()).unwrap();
        assert!(!fds.is_empty());
        // FDs 0, 1, 2 should exist.
        let stdin = fds.iter().find(|f| f.fd == 0);
        assert!(stdin.is_some());
    }

    #[test]
    fn missing_pid_errors() {
        let result = list_fds(999_999);
        assert!(matches!(
            result,
            Err(ProcessError::ProcessNotFound(999_999))
        ));
    }
}
