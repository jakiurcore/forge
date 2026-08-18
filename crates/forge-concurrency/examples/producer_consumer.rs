//! Multi-producer/single-consumer channel demonstration.

use forge_concurrency::channel::run_producer_consumer;

fn main() {
    let total = run_producer_consumer(4, 250);
    println!(
        "producers: 4, messages each: 250, total received: {}",
        total
    );
}
