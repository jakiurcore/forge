//! Curriculum loading and phase modeling.
//!
//! The curriculum is stored as a set of YAML files under `curriculum/`.
//! Each file describes one phase of the 300-day engineering laboratory.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// A single day entry in the curriculum.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Day {
    /// Global day number, 0 to 300.
    pub day: u32,
    /// Topic tag, e.g. `processes` or `threads`.
    pub topic: String,
    /// Human-readable title for the day.
    pub title: String,
    /// Activity type: `theory`, `implementation`, `experiment`, `review`, etc.
    #[serde(default)]
    pub r#type: String,
}

/// A phase of the curriculum.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Phase {
    /// Machine-readable phase identifier.
    pub phase: String,
    /// Human-readable phase name.
    pub name: String,
    /// Days belonging to this phase.
    #[serde(default)]
    pub days: Vec<Day>,
}

/// The full 300-day curriculum, composed of all phases.
#[derive(Debug, Clone)]
pub struct Curriculum {
    /// Map from phase identifier to phase definition.
    pub phases: BTreeMap<String, Phase>,
}

impl Curriculum {
    /// Total number of days in the curriculum.
    pub const TOTAL_DAYS: u32 = 300;

    /// Load the curriculum from a directory containing phase YAML files.
    pub fn load_from_dir<P: AsRef<Path>>(dir: P) -> anyhow::Result<Self> {
        let dir = dir.as_ref();
        let mut phases = BTreeMap::new();

        if !dir.exists() {
            anyhow::bail!("curriculum directory does not exist: {}", dir.display());
        }

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("yml") {
                let content = std::fs::read_to_string(&path)?;
                let phase: Phase = serde_yaml::from_str(&content)
                    .map_err(|e| anyhow::anyhow!("failed to parse {}: {}", path.display(), e))?;
                phases.insert(phase.phase.clone(), phase);
            }
        }

        if phases.is_empty() {
            anyhow::bail!("no curriculum phases found in {}", dir.display());
        }

        Ok(Curriculum { phases })
    }

    /// Load the default curriculum from the `curriculum/` directory relative to
    /// the current working directory.
    pub fn load_default() -> anyhow::Result<Self> {
        Self::load_from_dir("curriculum")
    }

    /// Resolve a human-readable phase name from a global day number.
    pub fn phase_for_day(&self, day: u32) -> Option<&Phase> {
        let phase_id = phase_id_for_day(day);
        self.phases.get(phase_id)
    }

    /// Return the phase that follows the given day, if any.
    pub fn next_phase_for_day(&self, day: u32) -> Option<&Phase> {
        let next_day = day.saturating_add(1);
        if next_day > Self::TOTAL_DAYS {
            return None;
        }
        let next_phase_id = phase_id_for_day(next_day);
        let current_phase_id = phase_id_for_day(day);
        if next_phase_id != current_phase_id {
            self.phases.get(next_phase_id)
        } else {
            None
        }
    }

    /// Return the ordered list of phase identifiers.
    pub fn phase_ids(&self) -> Vec<&String> {
        self.phases.keys().collect()
    }
}

/// Map a global day number to its phase identifier.
pub fn phase_id_for_day(day: u32) -> &'static str {
    match day {
        0 => "foundation",
        1..=100 => "atlas",
        101..=200 => "internet",
        201..=300 => "toolkit",
        _ => "unknown",
    }
}

/// Human-readable phase name.
pub fn phase_name(phase_id: &str) -> &'static str {
    match phase_id {
        "foundation" => "Foundation",
        "atlas" => "Engineering Atlas",
        "internet" => "Build the Internet",
        "toolkit" => "Developer Toolkit",
        _ => "Unknown",
    }
}

/// Path to the default curriculum directory relative to the workspace root.
pub fn default_curriculum_dir() -> PathBuf {
    PathBuf::from("curriculum")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn make_phase(id: &str, name: &str, start_day: u32, count: u32) -> Phase {
        let days = (0..count)
            .map(|i| Day {
                day: start_day + i,
                topic: id.to_string(),
                title: format!("Day {}", start_day + i),
                r#type: "theory".to_string(),
            })
            .collect();
        Phase {
            phase: id.to_string(),
            name: name.to_string(),
            days,
        }
    }

    #[test]
    fn load_from_dir_parses_yaml() {
        let tmp = tempfile::tempdir().unwrap();
        let phase = make_phase("foundation", "Foundation", 0, 1);
        let content = serde_yaml::to_string(&phase).unwrap();
        let mut file = std::fs::File::create(tmp.path().join("phase-0-foundation.yml")).unwrap();
        file.write_all(content.as_bytes()).unwrap();

        let curriculum = Curriculum::load_from_dir(tmp.path()).unwrap();
        assert_eq!(curriculum.phases.len(), 1);
        assert!(curriculum.phases.contains_key("foundation"));
    }

    #[test]
    fn phase_for_day_maps_correctly() {
        let tmp = tempfile::tempdir().unwrap();
        for (id, name, start, count) in [
            ("foundation", "Foundation", 0, 1),
            ("atlas", "Atlas", 1, 10),
        ] {
            let phase = make_phase(id, name, start, count);
            let content = serde_yaml::to_string(&phase).unwrap();
            let mut file =
                std::fs::File::create(tmp.path().join(format!("phase-{id}.yml"))).unwrap();
            file.write_all(content.as_bytes()).unwrap();
        }

        let curriculum = Curriculum::load_from_dir(tmp.path()).unwrap();
        assert_eq!(curriculum.phase_for_day(0).unwrap().phase, "foundation");
        assert_eq!(curriculum.phase_for_day(1).unwrap().phase, "atlas");
        assert_eq!(curriculum.phase_for_day(10).unwrap().phase, "atlas");
    }
}
