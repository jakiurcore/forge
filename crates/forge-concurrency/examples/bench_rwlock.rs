//! Compare read-heavy vs write-heavy RwLock contention.

use forge_concurrency::rwlocked::RwLockedCache;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

fn bench_read_heavy(readers: usize, reads_each: usize) -> Duration {
    let cache: Arc<RwLockedCache<String, i64>> = Arc::new(RwLockedCache::new());
    cache.insert("key".to_string(), 42);

    let start = Instant::now();
    let mut handles = Vec::new();
    for _ in 0..readers {
        let c = Arc::clone(&cache);
        handles.push(thread::spawn(move || {
            for _ in 0..reads_each {
                let _ = c.get(&"key".to_string());
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    start.elapsed()
}

fn bench_write_heavy(writers: usize, writes_each: usize) -> Duration {
    let cache: Arc<RwLockedCache<String, i64>> = Arc::new(RwLockedCache::new());

    let start = Instant::now();
    let mut handles = Vec::new();
    for i in 0..writers {
        let c = Arc::clone(&cache);
        handles.push(thread::spawn(move || {
            for j in 0..writes_each {
                c.insert(format!("key-{}-{}", i, j), j as i64);
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    start.elapsed()
}

fn main() {
    let read_time = bench_read_heavy(8, 100_000);
    let write_time = bench_write_heavy(8, 10_000);

    println!("read-heavy (8 readers x 100k): {:?}", read_time);
    println!("write-heavy (8 writers x 10k): {:?}", write_time);
}
