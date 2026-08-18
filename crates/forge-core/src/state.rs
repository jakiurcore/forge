//! Persistent Forge state.
//!
//! State is stored in TOML at `.forge/state.toml`. The file is gitignored so
//! that local progress does not pollute the repository. All writes are
//! idempotent: writing the same state twice produces the same file.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// The persistent state of the Forge laboratory.
#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ForgeState {
    /// The current Forge day, from 0 to 300.
    #[serde(default)]
    pub day: u32,

    /// Days that have been completed and validated.
    #[serde(default)]
    pub completed_days: Vec<u32>,

    /// ISO 8601 timestamp of the last state update.
    pub last_updated: Option<String>,
}

impl ForgeState {
    /// The default state file path.
    pub fn default_path() -> PathBuf {
        PathBuf::from(".forge/state.toml")
    }

    /// Load state from a file, returning defaults if the file does not exist.
    pub fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(path)?;
        let state: ForgeState = toml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("failed to parse state file {}: {}", path.display(), e))?;
        Ok(state)
    }

    /// Load the default state file.
    pub fn load_default() -> anyhow::Result<Self> {
        Self::load(Self::default_path())
    }

    /// Save state to a file. Creates parent directories if needed.
    pub fn save<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Save to the default state file.
    pub fn save_default(&self) -> anyhow::Result<()> {
        self.save(Self::default_path())
    }

    /// Mark a day as completed. Idempotent.
    pub fn complete_day(&mut self, day: u32) {
        if !self.completed_days.contains(&day) {
            self.completed_days.push(day);
            self.completed_days.sort_unstable();
        }
        self.day = self.day.max(day);
        self.touch();
    }

    /// Update the last-updated timestamp to now.
    pub fn touch(&mut self) {
        self.last_updated = Some(chrono::Utc::now().to_rfc3339());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_state_is_day_zero() {
        let state = ForgeState::default();
        assert_eq!(state.day, 0);
        assert!(state.completed_days.is_empty());
    }

    #[test]
    fn save_and_load_roundtrip() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("state.toml");
        let mut state = ForgeState::default();
        state.complete_day(0);
        state.save(&path).unwrap();

        let loaded = ForgeState::load(&path).unwrap();
        assert_eq!(loaded.day, 0);
        assert_eq!(loaded.completed_days, vec![0]);
        assert!(loaded.last_updated.is_some());
    }

    #[test]
    fn load_missing_returns_default() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("missing.toml");
        let state = ForgeState::load(&path).unwrap();
        assert_eq!(state, ForgeState::default());
    }

    #[test]
    fn complete_day_is_idempotent() {
        let mut state = ForgeState::default();
        state.complete_day(1);
        state.complete_day(1);
        assert_eq!(state.completed_days, vec![1]);
    }
}
