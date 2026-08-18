//! Forge core — curriculum, state, status, and watchdog models.
//!
//! This crate contains the engine that drives the 300-day Forge engineering
//! laboratory. It is intentionally dependency-light and focused on data
//! modeling, persistence, and reporting.

#![deny(missing_docs)]

pub mod curriculum;
pub mod state;
pub mod status;
pub mod watchdog;

/// Re-export common types.
pub use curriculum::Curriculum;
pub use state::ForgeState;
pub use status::StatusReport;
