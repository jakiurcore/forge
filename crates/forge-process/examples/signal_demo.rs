//! Demonstrates signal handling. Run and send SIGTERM or SIGINT.

use forge_process::signal::{common_signals, install_ignore_handler};
use nix::sys::signal::Signal;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Installed signal handlers. Common signals:");
    for (sig, desc) in common_signals() {
        println!("  {:?}: {}", sig, desc);
    }

    install_ignore_handler(Signal::SIGTERM).expect("sigterm handler");
    install_ignore_handler(Signal::SIGINT).expect("sigint handler");

    println!(
        "Send SIGTERM or SIGINT to pid {}. Waiting 10 seconds...",
        std::process::id()
    );
    for i in 0..10 {
        println!("  {}s", i);
        thread::sleep(Duration::from_secs(1));
    }
    println!("Done.");
}
