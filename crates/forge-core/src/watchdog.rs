//! Watchdog data model and bounded retry policy.
//!
//! The watchdog monitors the health of the Forge automation system and reports
//! failures. The bash implementation in `scripts/watchdog.sh` performs the
//! actual system checks; this module provides the shared model.

use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Categories of failure the watchdog can detect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureKind {
    /// The daily workflow did not complete.
    MissingDailyCompletion,
    /// The workspace build is broken.
    BrokenBuild,
    /// Validation (tests, fmt, clippy) failed.
    FailedValidation,
    /// State file is missing or corrupt.
    IncompleteState,
    /// An automation step failed without a more specific classification.
    AutomationFailure,
}

impl FailureKind {
    /// Human-readable label for the failure kind.
    pub fn label(&self) -> &'static str {
        match self {
            FailureKind::MissingDailyCompletion => "missing daily completion",
            FailureKind::BrokenBuild => "broken build",
            FailureKind::FailedValidation => "failed validation",
            FailureKind::IncompleteState => "incomplete state",
            FailureKind::AutomationFailure => "automation failure",
        }
    }
}

/// A single watchdog finding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    /// Category of failure.
    pub kind: FailureKind,
    /// Human-readable description.
    pub message: String,
    /// Unix timestamp when the finding was created.
    pub timestamp: u64,
}

impl Finding {
    /// Create a new finding with the current timestamp.
    pub fn new(kind: FailureKind, message: impl Into<String>) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            kind,
            message: message.into(),
            timestamp,
        }
    }
}

/// Bounded retry policy.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts before giving up.
    pub max_attempts: u32,
    /// Backoff in seconds.
    pub base_delay_secs: u64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay_secs: 60,
        }
    }
}

impl RetryPolicy {
    /// Compute the delay before the `attempt`-th retry (1-indexed).
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        if attempt == 0 || attempt > self.max_attempts {
            return Duration::ZERO;
        }
        let exponent = attempt.saturating_sub(1);
        let secs = self
            .base_delay_secs
            .saturating_mul(2u64.saturating_pow(exponent));
        Duration::from_secs(secs)
    }

    /// Whether another retry is allowed.
    pub fn can_retry(&self, attempt: u32) -> bool {
        attempt < self.max_attempts
    }
}

/// A watchdog report that can be serialized to JSON or Markdown.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WatchdogReport {
    /// Whether the watchdog detected any failures.
    pub healthy: bool,
    /// List of detected findings.
    pub findings: Vec<Finding>,
    /// Number of retries performed while generating the report.
    pub retries: u32,
}

impl WatchdogReport {
    /// Render the report as Markdown.
    pub fn markdown(&self) -> String {
        let mut lines = Vec::new();
        lines.push("# Forge Watchdog Report".to_string());
        lines.push(String::new());
        if self.healthy {
            lines.push("Status: **healthy**".to_string());
        } else {
            lines.push("Status: **unhealthy**".to_string());
        }
        lines.push(format!("Retries: {}", self.retries));
        lines.push(String::new());
        if self.findings.is_empty() {
            lines.push("No findings.".to_string());
        } else {
            lines.push("## Findings".to_string());
            for finding in &self.findings {
                lines.push(format!(
                    "- **{}**: {}",
                    finding.kind.label(),
                    finding.message
                ));
            }
        }
        lines.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retry_policy_backoff() {
        let policy = RetryPolicy::default();
        assert_eq!(policy.delay_for_attempt(1), Duration::from_secs(60));
        assert_eq!(policy.delay_for_attempt(2), Duration::from_secs(120));
        assert_eq!(policy.delay_for_attempt(3), Duration::from_secs(240));
    }

    #[test]
    fn can_retry_bounds() {
        let policy = RetryPolicy::default();
        assert!(policy.can_retry(0));
        assert!(policy.can_retry(1));
        assert!(policy.can_retry(2));
        assert!(!policy.can_retry(3));
    }

    #[test]
    fn healthy_report_markdown() {
        let report = WatchdogReport {
            healthy: true,
            findings: vec![],
            retries: 0,
        };
        let md = report.markdown();
        assert!(md.contains("healthy"));
        assert!(md.contains("No findings"));
    }
}
