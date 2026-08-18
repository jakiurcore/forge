//! A reusable thread pool.

use crate::error::ConcurrencyError;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

/// A job that can be executed by the pool.
type Job = Box<dyn FnOnce() + Send + 'static>;

/// A fixed-size pool of worker threads that execute submitted jobs.
#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<Sender<Job>>,
}

impl ThreadPool {
    /// Create a new pool with `size` worker threads.
    ///
    /// Returns an error if `size` is zero.
    pub fn new(size: usize) -> Result<Self, ConcurrencyError> {
        if size == 0 {
            return Err(ConcurrencyError::Other(
                "thread pool size must be greater than zero".to_string(),
            ));
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Ok(ThreadPool {
            workers,
            sender: Some(sender),
        })
    }

    /// Submit a job to the pool.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        if let Some(sender) = &self.sender {
            let job = Box::new(f);
            sender.send(job).ok();
        }
    }

    /// Number of worker threads.
    pub fn len(&self) -> usize {
        self.workers.len()
    }

    /// Whether the pool has no workers.
    pub fn is_empty(&self) -> bool {
        self.workers.is_empty()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.sender.take();
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().ok();
            }
        }
    }
}

#[derive(Debug)]
struct Worker {
    #[allow(dead_code)]
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv();
            match job {
                Ok(job) => job(),
                Err(_) => break,
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn pool_executes_jobs() {
        let pool = ThreadPool::new(4).unwrap();
        let counter = Arc::new(AtomicUsize::new(0));

        for _ in 0..100 {
            let c = Arc::clone(&counter);
            pool.execute(move || {
                c.fetch_add(1, Ordering::SeqCst);
            });
        }

        drop(pool);
        assert_eq!(counter.load(Ordering::SeqCst), 100);
    }

    #[test]
    fn zero_size_pool_errors() {
        assert!(ThreadPool::new(0).is_err());
    }
}
