//! Demonstrate sequential vs random memory access.

use forge_memory::locality::{bench_random, bench_sequential, random_indices};

fn main() {
    let len = 10_000_000;
    let data: Vec<u64> = (0..len as u64).collect();
    let indices = random_indices(len);

    let seq_time = bench_sequential(&data);
    let rand_time = bench_random(&data, &indices);

    println!("array length: {}", len);
    println!("sequential sum: {:?}", seq_time);
    println!("random sum: {:?}", rand_time);
    println!(
        "random/sequential ratio: {:.2}",
        rand_time.as_secs_f64() / seq_time.as_secs_f64()
    );
}
