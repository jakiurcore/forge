//! Measure the latency of fork().
//!
//! The child exits immediately; the parent waits and measures elapsed time.

use forge_process::fork::demo_fork;
use forge_process::wait::wait_for_child;
use std::time::Instant;

fn main() {
    let iterations: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000);

    let start = Instant::now();
    for _ in 0..iterations {
        match demo_fork().expect("fork failed") {
            forge_process::fork::ForkOutcome::Parent { child_pid } => {
                let _ = wait_for_child(child_pid);
            }
            forge_process::fork::ForkOutcome::Child => {
                std::process::exit(0);
            }
        }
    }
    let elapsed = start.elapsed();

    println!("iterations: {}", iterations);
    println!("total: {:?}", elapsed);
    println!("per fork+wait: {:?}", elapsed / iterations as u32);
}
