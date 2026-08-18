//! Safe deadlock demonstration with an escape hatch.
//!
//! This module is for education only. The demonstration uses two mutexes and
/// inconsistent lock ordering. A timeout prevents the demonstration from
/// hanging indefinitely, so it is safe to run manually but is not part of the
/// normal test suite.
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Two locks that can deadlock if acquired in opposite orders.
#[derive(Debug, Clone)]
pub struct DeadlockProneLocks {
    lock_a: Arc<Mutex<u64>>,
    lock_b: Arc<Mutex<u64>>,
}

impl Default for DeadlockProneLocks {
    fn default() -> Self {
        Self::new()
    }
}

impl DeadlockProneLocks {
    /// Create the pair of locks.
    pub fn new() -> Self {
        Self {
            lock_a: Arc::new(Mutex::new(0)),
            lock_b: Arc::new(Mutex::new(0)),
        }
    }

    /// Acquire A then B. This ordering is consistent.
    pub fn acquire_a_then_b(&self) -> (u64, u64) {
        let a = *self.lock_a.lock().unwrap();
        thread::sleep(Duration::from_millis(10));
        let b = *self.lock_b.lock().unwrap();
        (a, b)
    }

    /// Acquire B then A. This ordering is inconsistent with `acquire_a_then_b`
    /// and can deadlock.
    pub fn acquire_b_then_a(&self) -> (u64, u64) {
        let b = *self.lock_b.lock().unwrap();
        thread::sleep(Duration::from_millis(10));
        let a = *self.lock_a.lock().unwrap();
        (a, b)
    }
}

/// Run the deadlock demonstration with a timeout.
///
/// Returns `Some((result_a, result_b))` if it completes, or `None` if the
/// timeout fires first.
pub fn demonstrate_deadlock(timeout: Duration) -> Option<(u64, u64)> {
    let locks = Arc::new(DeadlockProneLocks::new());

    let locks1 = Arc::clone(&locks);
    let handle1 = thread::spawn(move || locks1.acquire_a_then_b());

    let locks2 = Arc::clone(&locks);
    let handle2 = thread::spawn(move || locks2.acquire_b_then_a());

    // Wait for both threads with an overall timeout.
    let start = std::time::Instant::now();
    while start.elapsed() < timeout {
        if handle1.is_finished() && handle2.is_finished() {
            let r1 = handle1.join().ok()?;
            let r2 = handle2.join().ok()?;
            return Some((r1.0 + r2.0, r1.1 + r2.1));
        }
        thread::sleep(Duration::from_millis(10));
    }

    // Timeout: the threads may still be deadlocked. We cannot safely join them
    // without potentially blocking forever, so we detach them and return None.
    None
}

/// Avoiding deadlock by always acquiring locks in the same order.
pub fn safe_acquisition(locks: &DeadlockProneLocks) -> (u64, u64) {
    let a = *locks.lock_a.lock().unwrap();
    let b = *locks.lock_b.lock().unwrap();
    (a, b)
}

// No tests in this module: deadlock demos are run manually with timeouts.
