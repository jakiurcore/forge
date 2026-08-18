//! Measure channel throughput.

use forge_concurrency::channel::run_producer_consumer;
use std::time::Instant;

fn main() {
    let producers = 4;
    let messages = 100_000;

    let start = Instant::now();
    let total = run_producer_consumer(producers, messages);
    let elapsed = start.elapsed();

    println!("producers: {}", producers);
    println!("messages per producer: {}", messages);
    println!("total received: {}", total);
    println!("elapsed: {:?}", elapsed);
    println!("per message: {:?}", elapsed / total as u32);
}
