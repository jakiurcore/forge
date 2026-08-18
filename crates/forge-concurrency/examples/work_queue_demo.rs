//! Demonstrates bounded work queue with a faster producer than consumer.

use forge_concurrency::queue::BoundedWorkQueue;
use std::time::{Duration, Instant};

fn main() {
    let queue = BoundedWorkQueue::new(4, 1).unwrap();
    let start = Instant::now();

    // Producer submits work faster than the single worker can process.
    for i in 0..20 {
        match queue.try_submit(move || {
            std::thread::sleep(Duration::from_millis(20));
            println!("processed task {}", i);
        }) {
            Ok(()) => println!("submitted task {}", i),
            Err(e) => println!("rejected task {}: {}", i, e),
        }
    }

    let metrics = queue.shutdown();
    println!(
        "completed {}/{} tasks in {:?}",
        metrics.completed,
        metrics.submitted,
        start.elapsed()
    );
}
