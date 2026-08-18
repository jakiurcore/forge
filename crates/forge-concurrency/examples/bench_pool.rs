//! Measure thread pool throughput.

use forge_concurrency::pool::ThreadPool;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;

fn main() {
    let workers = 4;
    let jobs = 100_000;

    let pool = ThreadPool::new(workers).unwrap();
    let counter = Arc::new(AtomicUsize::new(0));

    let start = Instant::now();
    for _ in 0..jobs {
        let c = Arc::clone(&counter);
        pool.execute(move || {
            c.fetch_add(1, Ordering::SeqCst);
        });
    }
    drop(pool);
    let elapsed = start.elapsed();

    println!("workers: {}", workers);
    println!("jobs: {}", jobs);
    println!("completed: {}", counter.load(Ordering::SeqCst));
    println!("elapsed: {:?}", elapsed);
    println!("per job: {:?}", elapsed / jobs as u32);
}
