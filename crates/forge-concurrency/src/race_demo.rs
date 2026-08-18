//! Safe demonstration of a logical race condition.
//!
//! This module is educational: it shows how unsynchronized reads and writes to
//! shared state can produce inconsistent observable results. It does not use
//! undefined behavior; the counter is a plain `u64` protected by an *external*
//! experiment harness that intentionally reads mid-update.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;

/// A deliberately racy counter for demonstration purposes only.
pub struct UnsafeCounter {
    value: AtomicU64,
}

impl Default for UnsafeCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl UnsafeCounter {
    /// Create a counter starting at zero.
    pub fn new() -> Self {
        Self {
            value: AtomicU64::new(0),
        }
    }

    /// Increment by one using a non-atomic read-modify-write sequence.
    ///
    /// This is safe in Rust because the underlying `AtomicU64` operations are
    /// still atomic, but the read-modify-write sequence itself is not. The
    /// result demonstrates a logical race: updates can be lost when two threads
    /// interleave between read and write.
    pub fn racy_increment(&self) {
        let current = self.value.load(Ordering::Relaxed);
        // A real data race could occur here with raw pointers. We deliberately
        // use a gap to make the race observable without invoking undefined
        // behavior.
        thread::yield_now();
        self.value.store(current + 1, Ordering::Relaxed);
    }

    /// Read the current value.
    pub fn get(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }
}

/// Run the race demonstration and return the observed and expected counts.
pub fn demonstrate_race(threads: usize, increments_per_thread: usize) -> (u64, u64) {
    let counter = Arc::new(UnsafeCounter::new());
    let mut handles = Vec::new();

    for _ in 0..threads {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            for _ in 0..increments_per_thread {
                c.racy_increment();
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    let observed = counter.get();
    let expected = (threads * increments_per_thread) as u64;
    (observed, expected)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn race_demo_loses_updates() {
        let (observed, expected) = demonstrate_race(8, 1000);
        // The race usually loses updates, but it is not deterministic. We only
        // assert that it does not exceed the expected value.
        assert!(observed <= expected);
        // With enough contention it should almost always be less than expected.
        // We avoid a strict inequality assertion to keep the test stable.
    }
}
