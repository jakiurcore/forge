//! Measure the latency of fork() + exec() for `/bin/true`.

use forge_process::exec::fork_exec;
use forge_process::wait::wait_for_child;
use std::time::Instant;

fn main() {
    let iterations: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(500);

    let start = Instant::now();
    for _ in 0..iterations {
        let child = fork_exec("/bin/true", &[]).expect("fork_exec failed");
        let _ = wait_for_child(child);
    }
    let elapsed = start.elapsed();

    println!("iterations: {}", iterations);
    println!("total: {:?}", elapsed);
    println!("per fork+exec+wait: {:?}", elapsed / iterations as u32);
}
