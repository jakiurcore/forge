//! Demonstrates unsynchronized vs mutex-protected shared mutation.

use forge_concurrency::counter::SharedCounter;
use forge_concurrency::race_demo::demonstrate_race;

fn main() {
    println!("--- Unsafe racy increments ---");
    let (observed, expected) = demonstrate_race(8, 1000);
    println!("observed: {}, expected: {}", observed, expected);

    println!("\n--- Mutex-protected increments ---");
    let counter = SharedCounter::new();
    let mut handles = Vec::new();
    for _ in 0..8 {
        let c = counter.clone();
        handles.push(std::thread::spawn(move || {
            for _ in 0..1000 {
                c.increment();
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("mutex counter: {} (expected 8000)", counter.get());
}
