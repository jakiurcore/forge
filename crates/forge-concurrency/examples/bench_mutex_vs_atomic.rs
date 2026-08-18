//! Compare mutex counter vs atomic counter under contention.

use forge_concurrency::atomic::AtomicCounter;
use forge_concurrency::counter::SharedCounter;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

fn bench_mutex(threads: usize, increments: usize) -> Duration {
    let counter = SharedCounter::new();
    let start = Instant::now();
    let mut handles = Vec::new();
    for _ in 0..threads {
        let c = counter.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..increments {
                c.increment();
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    start.elapsed()
}

fn bench_atomic(threads: usize, increments: usize) -> Duration {
    let counter = Arc::new(AtomicCounter::new());
    let start = Instant::now();
    let mut handles = Vec::new();
    for _ in 0..threads {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..increments {
                c.increment();
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    start.elapsed()
}

fn main() {
    let threads = 8;
    let increments = 100_000;

    let mutex_time = bench_mutex(threads, increments);
    let atomic_time = bench_atomic(threads, increments);

    println!("threads: {}", threads);
    println!("increments per thread: {}", increments);
    println!("mutex total: {:?}", mutex_time);
    println!("atomic total: {:?}", atomic_time);
    println!(
        "mutex per increment: {:?}",
        mutex_time / (threads * increments) as u32
    );
    println!(
        "atomic per increment: {:?}",
        atomic_time / (threads * increments) as u32
    );
}
