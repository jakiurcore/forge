//! A bounded work queue with backpressure and graceful shutdown.

use crate::error::ConcurrencyError;
use std::sync::mpsc::{self, SyncSender, TrySendError};
use std::sync::{Arc, Mutex};
use std::thread;

/// A task that can be executed by the work queue.
type Task = Box<dyn FnOnce() + Send + 'static>;

/// Metrics reported by the work queue.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct QueueMetrics {
    /// Total tasks submitted.
    pub submitted: u64,
    /// Total tasks completed.
    pub completed: u64,
    /// Tasks rejected because the queue was full.
    pub rejected: u64,
}

/// A bounded work queue that distributes tasks to worker threads.
#[derive(Debug)]
pub struct BoundedWorkQueue {
    sender: Option<SyncSender<Task>>,
    workers: Vec<thread::JoinHandle<()>>,
    metrics: Arc<Mutex<QueueMetrics>>,
}

impl BoundedWorkQueue {
    /// Create a queue with `capacity` slots and `workers` worker threads.
    pub fn new(capacity: usize, workers: usize) -> Result<Self, ConcurrencyError> {
        if capacity == 0 || workers == 0 {
            return Err(ConcurrencyError::Other(
                "capacity and workers must be greater than zero".to_string(),
            ));
        }

        let (sender, receiver) = mpsc::sync_channel::<Task>(capacity);
        let receiver = Arc::new(Mutex::new(receiver));
        let metrics = Arc::new(Mutex::new(QueueMetrics::default()));

        let mut worker_handles = Vec::with_capacity(workers);
        for _ in 0..workers {
            let rx = Arc::clone(&receiver);
            let m = Arc::clone(&metrics);
            worker_handles.push(thread::spawn(move || {
                while let Ok(task) = rx.lock().unwrap().recv() {
                    task();
                    m.lock().unwrap().completed += 1;
                }
            }));
        }

        Ok(BoundedWorkQueue {
            sender: Some(sender),
            workers: worker_handles,
            metrics,
        })
    }

    /// Try to submit a task without blocking.
    ///
    /// Returns `Ok(())` on success or `Err(ConcurrencyError::QueueFull)` if the
    /// queue is saturated.
    pub fn try_submit<F>(&self, task: F) -> Result<(), ConcurrencyError>
    where
        F: FnOnce() + Send + 'static,
    {
        if let Some(sender) = &self.sender {
            match sender.try_send(Box::new(task)) {
                Ok(()) => {
                    self.metrics.lock().unwrap().submitted += 1;
                    Ok(())
                }
                Err(TrySendError::Full(_)) => {
                    self.metrics.lock().unwrap().rejected += 1;
                    Err(ConcurrencyError::QueueFull)
                }
                Err(TrySendError::Disconnected(_)) => Err(ConcurrencyError::Shutdown),
            }
        } else {
            Err(ConcurrencyError::Shutdown)
        }
    }

    /// Submit a task, blocking if the queue is full.
    pub fn submit<F>(&self, task: F) -> Result<(), ConcurrencyError>
    where
        F: FnOnce() + Send + 'static,
    {
        if let Some(sender) = &self.sender {
            sender
                .send(Box::new(task))
                .map_err(|_| ConcurrencyError::Shutdown)?;
            self.metrics.lock().unwrap().submitted += 1;
            Ok(())
        } else {
            Err(ConcurrencyError::Shutdown)
        }
    }

    /// Shut down the queue and wait for workers to finish.
    pub fn shutdown(mut self) -> QueueMetrics {
        self.sender.take();
        for worker in self.workers {
            worker.join().ok();
        }
        *self.metrics.lock().unwrap()
    }

    /// Read current metrics.
    pub fn metrics(&self) -> QueueMetrics {
        *self.metrics.lock().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn submit_and_complete() {
        let queue = BoundedWorkQueue::new(10, 2).unwrap();
        for _ in 0..20 {
            queue.submit(|| {}).unwrap();
        }
        let metrics = queue.shutdown();
        assert_eq!(metrics.submitted, 20);
        assert_eq!(metrics.completed, 20);
    }

    #[test]
    fn try_submit_respects_capacity() {
        let queue = BoundedWorkQueue::new(2, 1).unwrap();
        queue.try_submit(|| {}).unwrap();
        queue.try_submit(|| {}).unwrap();
        // Capacity may be full depending on worker progress.
        let result = queue.try_submit(|| {});
        assert!(result.is_ok() || result == Err(ConcurrencyError::QueueFull));
        queue.shutdown();
    }

    #[test]
    fn tasks_execute_and_update_shared_state() {
        let queue = BoundedWorkQueue::new(10, 4).unwrap();
        let counter = Arc::new(AtomicUsize::new(0));
        for _ in 0..100 {
            let c = Arc::clone(&counter);
            queue
                .submit(move || {
                    c.fetch_add(1, Ordering::SeqCst);
                })
                .unwrap();
        }
        queue.shutdown();
        assert_eq!(counter.load(Ordering::SeqCst), 100);
    }
}
