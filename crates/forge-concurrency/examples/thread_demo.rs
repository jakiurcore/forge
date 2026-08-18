//! Demonstrates spawning and joining threads.

use std::thread;

fn main() {
    let mut handles = Vec::new();
    for i in 0..4 {
        handles.push(thread::spawn(move || {
            println!("thread {} running", i);
            i * i
        }));
    }

    for h in handles {
        let result = h.join().unwrap();
        println!("thread returned {}", result);
    }
}
