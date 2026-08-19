//! Forge CLI — the command-line interface for the engineering laboratory.

use anyhow::Context;
use clap::{Parser, Subcommand};
use forge_core::curriculum::Curriculum;
use forge_core::state::ForgeState;
use forge_core::status::StatusReport;
use std::time::Duration;

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
    /// Inspect operating system processes (Linux-specific).
    Process {
        #[command(subcommand)]
        action: ProcessAction,
    },
    /// Concurrency demonstrations and experiments.
    Concurrency {
        #[command(subcommand)]
        action: ConcurrencyAction,
    },
    /// Memory management utilities and inspection.
    Memory {
        #[command(subcommand)]
        action: MemoryAction,
    },
    /// Networking utilities and diagnostics.
    Network {
        #[command(subcommand)]
        action: NetworkAction,
    },
}

#[derive(Debug, Subcommand)]
enum ProcessAction {
    /// Inspect a process by PID.
    Inspect { pid: u32 },
    /// List open file descriptors for a process.
    Fds { pid: u32 },
}

#[derive(Debug, Subcommand)]
enum ConcurrencyAction {
    /// Demonstrate a safe logical race condition.
    RaceDemo,
    /// Demonstrate deadlock with a timeout.
    DeadlockDemo,
}

#[derive(Debug, Subcommand)]
enum MemoryAction {
    /// Report the system page size.
    PageSize,
    /// Inspect memory mappings of a process.
    Inspect { pid: u32 },
}

#[derive(Debug, Subcommand)]
enum NetworkAction {
    /// Run a TCP echo server.
    TcpEcho {
        /// Address to bind to.
        #[arg(long, default_value = "127.0.0.1:0")]
        bind: String,
    },
    /// Send a message to a TCP echo server and print the reply.
    TcpConnect {
        /// Server address.
        addr: String,
        /// Message to send.
        #[arg(short, long, default_value = "hello, forge")]
        message: String,
    },
    /// Run a UDP echo server.
    UdpEcho {
        /// Address to bind to.
        #[arg(long, default_value = "127.0.0.1:0")]
        bind: String,
    },
    /// List local network interfaces and addresses.
    Inspect,
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
        Command::Process { action } => match action {
            ProcessAction::Inspect { pid } => {
                let snapshot = forge_process::ProcessSnapshot::inspect(pid)
                    .map_err(|e| anyhow::anyhow!("failed to inspect process {}: {}", pid, e))?;
                println!("{}", snapshot.render());
                Ok(())
            }
            ProcessAction::Fds { pid } => {
                let snapshot = forge_process::ProcessSnapshot::inspect(pid)
                    .map_err(|e| anyhow::anyhow!("failed to inspect process {}: {}", pid, e))?;
                println!("{}", snapshot.render_fds());
                Ok(())
            }
        },
        Command::Concurrency { action } => match action {
            ConcurrencyAction::RaceDemo => {
                let (observed, expected) = forge_concurrency::race_demo::demonstrate_race(8, 1000);
                println!("observed: {}, expected: {}", observed, expected);
                if observed < expected {
                    println!("race condition lost {} updates", expected - observed);
                }
                Ok(())
            }
            ConcurrencyAction::DeadlockDemo => {
                match forge_concurrency::deadlock_demo::demonstrate_deadlock(Duration::from_secs(1))
                {
                    Some(_) => println!("completed without deadlock"),
                    None => println!("deadlock detected (timed out)"),
                }
                Ok(())
            }
        },
        Command::Memory { action } => match action {
            MemoryAction::PageSize => {
                println!("{}", forge_memory::pages::page_size());
                Ok(())
            }
            MemoryAction::Inspect { pid } => {
                let maps = forge_memory::info::read_maps(pid).map_err(|e| {
                    anyhow::anyhow!("failed to read memory maps for {}: {}", pid, e)
                })?;
                println!("{:<20} {:<10} {:>10} PATHNAME", "RANGE", "PERMS", "OFFSET");
                for m in maps {
                    println!(
                        "{:016x}-{:016x} {:<10} {:>10} {}",
                        m.start, m.end, m.perms, m.offset, m.pathname
                    );
                }
                Ok(())
            }
        },
        Command::Network { action } => match action {
            NetworkAction::TcpEcho { bind } => {
                println!("starting TCP echo server on {}", bind);
                forge_network::tcp::run_echo_server(&bind)
                    .map_err(|e| anyhow::anyhow!("tcp echo server failed: {}", e))?;
                Ok(())
            }
            NetworkAction::TcpConnect { addr, message } => {
                let response = forge_network::tcp::send_message(
                    addr,
                    message.as_bytes(),
                    Duration::from_secs(2),
                    Duration::from_secs(2),
                )
                .map_err(|e| anyhow::anyhow!("tcp connect failed: {}", e))?;
                println!("{}", String::from_utf8_lossy(&response));
                Ok(())
            }
            NetworkAction::UdpEcho { bind } => {
                let (socket, local) = forge_network::udp::bind_udp(&bind)
                    .map_err(|e| anyhow::anyhow!("failed to bind udp socket: {}", e))?;
                println!("UDP echo server listening on {}", local);
                let mut buf = vec![0u8; 65535];
                loop {
                    let (n, peer) = socket
                        .recv_from(&mut buf)
                        .map_err(|e| anyhow::anyhow!("udp recv failed: {}", e))?;
                    socket
                        .send_to(&buf[..n], peer)
                        .map_err(|e| anyhow::anyhow!("udp send failed: {}", e))?;
                }
            }
            NetworkAction::Inspect => {
                let ifaces = forge_network::address::local_interfaces()
                    .map_err(|e| anyhow::anyhow!("failed to list interfaces: {}", e))?;
                if ifaces.is_empty() {
                    println!("no interfaces found (unsupported platform)");
                } else {
                    for iface in ifaces {
                        println!("{}", iface);
                    }
                }
                Ok(())
            }
        },
    }
}
