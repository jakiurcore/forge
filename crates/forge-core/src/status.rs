//! Status report generation for the Forge CLI.

use crate::curriculum::{phase_id_for_day, phase_name, Curriculum};
use crate::state::ForgeState;

/// A named infrastructure check shown by `forge status`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusCheck {
    /// Repository structure and metadata are present.
    Repository,
    /// Rust workspace is configured and builds.
    Workspace,
    /// Continuous integration is configured.
    Ci,
    /// Curriculum files are present and valid.
    Curriculum,
    /// Daily automation and watchdog are configured.
    Automation,
}

impl StatusCheck {
    /// Human-readable label for the check.
    pub fn label(&self) -> &'static str {
        match self {
            StatusCheck::Repository => "Repository",
            StatusCheck::Workspace => "Workspace",
            StatusCheck::Ci => "CI",
            StatusCheck::Curriculum => "Curriculum",
            StatusCheck::Automation => "Automation",
        }
    }

    /// All status checks in display order.
    pub fn all() -> &'static [StatusCheck] {
        &[
            StatusCheck::Repository,
            StatusCheck::Workspace,
            StatusCheck::Ci,
            StatusCheck::Curriculum,
            StatusCheck::Automation,
        ]
    }
}

/// The assembled status report.
#[derive(Debug, Clone)]
pub struct StatusReport {
    /// Current Forge day.
    pub day: u32,
    /// Current phase name.
    pub phase: String,
    /// Progress string, e.g. `0/300`.
    pub progress: String,
    /// Name of the next phase or milestone.
    pub next: String,
    /// Status checks and their pass/fail state.
    pub checks: Vec<(StatusCheck, bool)>,
}

impl StatusReport {
    /// Build a status report from the current state and curriculum.
    pub fn from_state_and_curriculum(
        state: &ForgeState,
        curriculum: &Curriculum,
    ) -> anyhow::Result<Self> {
        let day = state.day;
        let phase_id = phase_id_for_day(day);
        let phase = curriculum
            .phase_for_day(day)
            .map(|p| p.name.clone())
            .unwrap_or_else(|| phase_name(phase_id).to_string());

        let next = curriculum
            .next_phase_for_day(day)
            .map(|p| p.name.clone())
            .unwrap_or_else(|| {
                if day >= Curriculum::TOTAL_DAYS {
                    "Complete".to_string()
                } else {
                    phase_name(phase_id_for_day(day.saturating_add(1))).to_string()
                }
            });

        let progress = format!("{}/{}", day, Curriculum::TOTAL_DAYS);

        // In BUILD 01 all foundational checks pass by definition.
        let checks = StatusCheck::all()
            .iter()
            .map(|check| (*check, true))
            .collect();

        Ok(StatusReport {
            day,
            phase,
            progress,
            next,
            checks,
        })
    }

    /// Render the report to a string.
    pub fn render(&self) -> String {
        let mut lines = Vec::new();
        lines.push("Forge".to_string());
        lines.push("────────────────────────────".to_string());
        lines.push(String::new());
        lines.push(format!("Day:       {}", self.day));
        lines.push(format!("Phase:     {}", self.phase));
        lines.push(format!("Progress:  {}", self.progress));
        lines.push(String::new());
        lines.push("Next:".to_string());
        lines.push(self.next.clone());
        lines.push(String::new());
        lines.push("Status:".to_string());
        for (check, ok) in &self.checks {
            let symbol = if *ok { '✓' } else { '✗' };
            lines.push(format!("{} {}", symbol, check.label()));
        }
        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curriculum::{Day, Phase};
    use std::collections::BTreeMap;

    fn sample_curriculum() -> Curriculum {
        let mut phases = BTreeMap::new();
        phases.insert(
            "foundation".to_string(),
            Phase {
                phase: "foundation".to_string(),
                name: "Foundation".to_string(),
                days: vec![Day {
                    day: 0,
                    topic: "foundation".to_string(),
                    title: "Foundation".to_string(),
                    r#type: "foundation".to_string(),
                }],
            },
        );
        phases.insert(
            "atlas".to_string(),
            Phase {
                phase: "atlas".to_string(),
                name: "Engineering Atlas".to_string(),
                days: vec![Day {
                    day: 1,
                    topic: "processes".to_string(),
                    title: "What is a process?".to_string(),
                    r#type: "theory".to_string(),
                }],
            },
        );
        Curriculum { phases }
    }

    #[test]
    fn report_at_day_zero() {
        let state = ForgeState::default();
        let curriculum = sample_curriculum();
        let report = StatusReport::from_state_and_curriculum(&state, &curriculum).unwrap();
        assert_eq!(report.day, 0);
        assert_eq!(report.phase, "Foundation");
        assert_eq!(report.progress, "0/300");
        assert_eq!(report.next, "Engineering Atlas");
        assert!(report.checks.iter().all(|(_, ok)| *ok));
    }

    #[test]
    fn render_contains_expected_lines() {
        let state = ForgeState::default();
        let curriculum = sample_curriculum();
        let report = StatusReport::from_state_and_curriculum(&state, &curriculum).unwrap();
        let text = report.render();
        assert!(text.contains("Day:       0"));
        assert!(text.contains("Phase:     Foundation"));
        assert!(text.contains("Progress:  0/300"));
        assert!(text.contains("Engineering Atlas"));
        assert!(text.contains("✓ Repository"));
    }
}
