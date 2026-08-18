//! Forge CLI — the command-line interface for the engineering laboratory.

use anyhow::Context;
use clap::{Parser, Subcommand};
use forge_core::curriculum::Curriculum;
use forge_core::state::ForgeState;
use forge_core::status::StatusReport;

/// Forge — a 300-day autonomous engineering laboratory.
#[derive(Debug, Parser)]
#[command(name = "forge", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Show the current Forge status.
    Status,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Status => {
            let curriculum = Curriculum::load_default().context("failed to load curriculum")?;
            let state = ForgeState::load_default().context("failed to load Forge state")?;
            let report = StatusReport::from_state_and_curriculum(&state, &curriculum)
                .context("failed to build status report")?;
            println!("{}", report.render());
            Ok(())
        }
    }
}
