//! Thread-safe counter protected by a mutex.

use std::sync::{Arc, Mutex};

/// A counter shared safely between threads.
#[derive(Debug, Clone)]
pub struct SharedCounter {
    value: Arc<Mutex<u64>>,
}

impl Default for SharedCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl SharedCounter {
    /// Create a new counter starting at zero.
    pub fn new() -> Self {
        Self {
            value: Arc::new(Mutex::new(0)),
        }
    }

    /// Create a counter with an initial value.
    pub fn with_value(initial: u64) -> Self {
        Self {
            value: Arc::new(Mutex::new(initial)),
        }
    }

    /// Atomically increment by one and return the new value.
    pub fn increment(&self) -> u64 {
        let mut guard = self.value.lock().unwrap();
        *guard += 1;
        *guard
    }

    /// Add a delta and return the new value.
    pub fn add(&self, delta: u64) -> u64 {
        let mut guard = self.value.lock().unwrap();
        *guard += delta;
        *guard
    }

    /// Read the current value.
    pub fn get(&self) -> u64 {
        *self.value.lock().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn single_threaded_increment() {
        let counter = SharedCounter::new();
        assert_eq!(counter.increment(), 1);
        assert_eq!(counter.increment(), 2);
        assert_eq!(counter.get(), 2);
    }

    #[test]
    fn concurrent_increments_are_correct() {
        let counter = SharedCounter::new();
        let mut handles = Vec::new();
        for _ in 0..10 {
            let c = counter.clone();
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
    fn concurrent_add_is_correct() {
        let counter = SharedCounter::new();
        let mut handles = Vec::new();
        for _ in 0..4 {
            let c = counter.clone();
            handles.push(thread::spawn(move || {
                c.add(100);
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(counter.get(), 400);
    }
}
