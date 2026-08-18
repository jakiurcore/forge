//! Producer/consumer helpers using `std::sync::mpsc`.

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;

/// Run a multi-producer, single-consumer experiment.
///
/// Each producer sends `messages_per_producer` values. The single consumer
/// receives values until all senders are dropped. Returns the total number of
/// messages received.
pub fn run_producer_consumer(producers: usize, messages_per_producer: usize) -> usize {
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    let total = Arc::new(Mutex::new(0usize));
    let mut handles = Vec::new();

    for _ in 0..producers {
        let sender = tx.clone();
        handles.push(thread::spawn(move || {
            for i in 0..messages_per_producer {
                sender.send(i).ok();
            }
        }));
    }
    drop(tx);

    let consumer_total = Arc::clone(&total);
    let consumer_rx = Arc::clone(&rx);
    handles.push(thread::spawn(move || {
        while let Ok(_msg) = consumer_rx.lock().unwrap().recv() {
            *consumer_total.lock().unwrap() += 1;
        }
    }));

    for h in handles {
        h.join().unwrap();
    }

    let value = *total.lock().unwrap();
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_producer_single_consumer() {
        let total = run_producer_consumer(1, 100);
        assert_eq!(total, 100);
    }

    #[test]
    fn multi_producer_single_consumer() {
        let total = run_producer_consumer(4, 250);
        assert_eq!(total, 1000);
    }
}
