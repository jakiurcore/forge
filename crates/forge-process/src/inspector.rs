//! High-level process inspector that combines `/proc` parsing into a usable API.

use crate::error::ProcessError;
use crate::fds::{list_fds, FdInfo};
use crate::info::ProcessInfo;

/// Combined view of a process.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessSnapshot {
    /// Basic process information.
    pub info: ProcessInfo,
    /// Open file descriptors.
    pub fds: Vec<FdInfo>,
}

impl ProcessSnapshot {
    /// Build a snapshot for a given PID.
    pub fn inspect(pid: u32) -> Result<Self, ProcessError> {
        let info = ProcessInfo::from_pid(pid)?;
        let fds = list_fds(pid)?;
        Ok(ProcessSnapshot { info, fds })
    }

    /// Render the snapshot as human-readable text.
    pub fn render(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!("PID:      {}", self.info.pid));
        lines.push(format!("PPID:     {}", self.info.ppid));
        lines.push(format!("Name:     {}", self.info.name));
        lines.push(format!("State:    {}", self.info.state.label()));
        if let Some(uid) = self.info.uid {
            lines.push(format!("UID:      {}", uid));
        }
        if let Some(gid) = self.info.gid {
            lines.push(format!("GID:      {}", gid));
        }
        if let Some(threads) = self.info.threads {
            lines.push(format!("Threads:  {}", threads));
        }
        if let Some(vmsize) = self.info.vmsize {
            lines.push(format!("VmSize:   {} bytes", vmsize));
        }
        if let Some(rss) = self.info.rss {
            lines.push(format!("VmRSS:    {} bytes", rss));
        }
        if !self.info.cmdline.is_empty() {
            lines.push(format!("Cmdline:  {}", self.info.cmdline.join(" ")));
        }
        lines.join("\n")
    }

    /// Render the file descriptor table.
    pub fn render_fds(&self) -> String {
        let mut lines = Vec::new();
        lines.push("FD   TYPE       TARGET".to_string());
        for fd in self.info_fds() {
            lines.push(format!(
                "{:<4} {:<10} {}",
                fd.fd,
                format!("{:?}", fd.kind).to_lowercase(),
                fd.target
            ));
        }
        lines.join("\n")
    }

    /// Borrow FDs for display.
    pub fn info_fds(&self) -> &[FdInfo] {
        &self.fds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inspect_self() {
        let snapshot = ProcessSnapshot::inspect(std::process::id()).unwrap();
        assert_eq!(snapshot.info.pid, std::process::id());
        assert!(!snapshot.render().is_empty());
        assert!(!snapshot.info_fds().is_empty());
    }
}
