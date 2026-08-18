//! Inspect process memory mappings via `/proc`.

use crate::error::MemoryError;
use std::fs;

/// A single memory mapping entry from `/proc/<pid>/maps`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryMapping {
    /// Start address (hex string).
    pub start: u64,
    /// End address (exclusive).
    pub end: u64,
    /// Permissions, e.g. "r-xp".
    pub perms: String,
    /// Offset into the file, if file-backed.
    pub offset: u64,
    /// Device major:minor.
    pub dev: String,
    /// Inode number.
    pub inode: u64,
    /// Path or descriptor, e.g. "[stack]" or "/bin/bash".
    pub pathname: String,
}

/// Read and parse `/proc/<pid>/maps`.
pub fn read_maps(pid: u32) -> Result<Vec<MemoryMapping>, MemoryError> {
    let path = format!("/proc/{}/maps", pid);
    let content = fs::read_to_string(&path)
        .map_err(|e| MemoryError::SystemError(format!("failed to read {}: {}", path, e)))?;
    parse_maps(&content)
}

/// Parse the contents of a `/proc/<pid>/maps` file.
pub fn parse_maps(content: &str) -> Result<Vec<MemoryMapping>, MemoryError> {
    let mut mappings = Vec::new();
    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 5 {
            continue;
        }
        let range = parts[0];
        let mut range_parts = range.split('-');
        let start = u64::from_str_radix(range_parts.next().unwrap_or("0"), 16)
            .map_err(|_| MemoryError::Other(format!("invalid address range: {}", range)))?;
        let end = u64::from_str_radix(range_parts.next().unwrap_or("0"), 16)
            .map_err(|_| MemoryError::Other(format!("invalid address range: {}", range)))?;

        let perms = parts[1].to_string();
        let offset = u64::from_str_radix(parts[2], 16)
            .map_err(|_| MemoryError::Other(format!("invalid offset: {}", parts[2])))?;
        let dev = parts[3].to_string();
        let inode = parts[4]
            .parse()
            .map_err(|_| MemoryError::Other(format!("invalid inode: {}", parts[4])))?;
        let pathname = if parts.len() > 5 {
            parts[5..].join(" ")
        } else {
            String::new()
        };

        mappings.push(MemoryMapping {
            start,
            end,
            perms,
            offset,
            dev,
            inode,
            pathname,
        });
    }
    Ok(mappings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sample_maps() {
        let sample = "00400000-00452000 r-xp 00000000 08:01 4194480 /bin/cat\n\
                      00651000-00652000 rw-p 00051000 08:01 4194480 /bin/cat\n\
                      7ffd12345000-7ffd12366000 rw-p 00000000 00:00 0 [stack]";
        let maps = parse_maps(sample).unwrap();
        assert_eq!(maps.len(), 3);
        assert_eq!(maps[0].pathname, "/bin/cat");
        assert_eq!(maps[2].pathname, "[stack]");
    }

    #[test]
    fn read_self_maps() {
        let maps = read_maps(std::process::id()).unwrap();
        assert!(!maps.is_empty());
    }
}
