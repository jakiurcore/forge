//! Measure pipe throughput between parent and child.

use forge_process::pipe::demo_pipe_message;
use std::time::Instant;

fn main() {
    let chunk_size: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(4096);
    let iterations: usize = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000);

    let message = vec![b'x'; chunk_size];
    let start = Instant::now();
    for _ in 0..iterations {
        let received = demo_pipe_message(&message).expect("pipe failed");
        assert_eq!(received.len(), chunk_size);
    }
    let elapsed = start.elapsed();

    let total_bytes = chunk_size * iterations;
    let seconds = elapsed.as_secs_f64();
    let mb_per_sec = (total_bytes as f64 / (1024.0 * 1024.0)) / seconds;

    println!("chunk_size: {} bytes", chunk_size);
    println!("iterations: {}", iterations);
    println!("total_bytes: {}", total_bytes);
    println!("total: {:?}", elapsed);
    println!("throughput: {:.2} MiB/s", mb_per_sec);
}
