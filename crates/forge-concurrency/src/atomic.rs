//! Atomic primitives and safe lock-free coordination helpers.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

/// A counter backed by an atomic integer.
#[derive(Debug)]
pub struct AtomicCounter {
    value: AtomicU64,
}

impl Default for AtomicCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl AtomicCounter {
    /// Create a counter starting at zero.
    pub fn new() -> Self {
        Self {
            value: AtomicU64::new(0),
        }
    }

    /// Create a counter with an initial value.
    pub fn with_value(initial: u64) -> Self {
        Self {
            value: AtomicU64::new(initial),
        }
    }

    /// Add one and return the previous value.
    pub fn fetch_add(&self, delta: u64) -> u64 {
        self.value.fetch_add(delta, Ordering::SeqCst)
    }

    /// Increment by one.
    pub fn increment(&self) -> u64 {
        self.fetch_add(1) + 1
    }

    /// Read the current value.
    pub fn get(&self) -> u64 {
        self.value.load(Ordering::SeqCst)
    }
}

/// A boolean flag that can be set and checked across threads.
#[derive(Debug)]
pub struct AtomicFlag {
    value: AtomicBool,
}

impl Default for AtomicFlag {
    fn default() -> Self {
        Self::new()
    }
}

impl AtomicFlag {
    /// Create a flag starting as false.
    pub fn new() -> Self {
        Self {
            value: AtomicBool::new(false),
        }
    }

    /// Set the flag to true.
    pub fn set(&self) {
        self.value.store(true, Ordering::SeqCst);
    }

    /// Check whether the flag is set.
    pub fn is_set(&self) -> bool {
        self.value.load(Ordering::SeqCst)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn atomic_counter_concurrent_increments() {
        let counter = Arc::new(AtomicCounter::new());
        let mut handles = Vec::new();
        for _ in 0..10 {
            let c = Arc::clone(&counter);
            handles.push(thread::spawn(move || {
                for _ in 0..1000 {
                    c.increment();
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(counter.get(), 10_000);
    }

    #[test]
    fn atomic_flag_set_and_check() {
        let flag = AtomicFlag::new();
        assert!(!flag.is_set());
        flag.set();
        assert!(flag.is_set());
    }
}
