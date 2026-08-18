//! Reusable concurrency primitives and experiments for Forge.
//!
//! This crate contains thread-safe counters, locks, channels, a thread pool,
//! a bounded work queue, and safe demonstrations of races and deadlocks.
//! All code uses safe Rust and standard library primitives unless otherwise
//! noted.

#![deny(missing_docs)]

pub mod atomic;
pub mod channel;
pub mod counter;
pub mod deadlock_demo;
pub mod error;
pub mod pool;
pub mod queue;
pub mod race_demo;
pub mod rwlocked;

/// Re-export commonly used types.
pub use atomic::{AtomicCounter, AtomicFlag};
pub use channel::run_producer_consumer;
pub use counter::SharedCounter;
pub use deadlock_demo::{demonstrate_deadlock, safe_acquisition, DeadlockProneLocks};
pub use pool::ThreadPool;
pub use queue::{BoundedWorkQueue, QueueMetrics};
pub use race_demo::{demonstrate_race, UnsafeCounter};
pub use rwlocked::RwLockedCache;
