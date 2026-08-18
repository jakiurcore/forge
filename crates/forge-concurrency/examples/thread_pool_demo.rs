//! Demonstrates the thread pool.

use forge_concurrency::pool::ThreadPool;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

fn main() {
    let pool = ThreadPool::new(4).unwrap();
    let counter = Arc::new(AtomicUsize::new(0));

    for _ in 0..20 {
        let c = Arc::clone(&counter);
        pool.execute(move || {
            c.fetch_add(1, Ordering::SeqCst);
        });
    }

    drop(pool);
    println!("jobs executed: {}", counter.load(Ordering::SeqCst));
}
