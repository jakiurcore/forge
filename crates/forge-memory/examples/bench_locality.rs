//! Benchmark sequential vs random access across sizes.

use forge_memory::locality::{bench_random, bench_sequential, random_indices};

fn main() {
    for &len in &[100_000, 1_000_000, 10_000_000] {
        let data: Vec<u64> = (0..len as u64).collect();
        let indices = random_indices(len);

        let seq_time = bench_sequential(&data);
        let rand_time = bench_random(&data, &indices);

        println!("len: {}", len);
        println!("  sequential: {:?}", seq_time);
        println!("  random: {:?}", rand_time);
        println!(
            "  ratio: {:.2}",
            rand_time.as_secs_f64() / seq_time.as_secs_f64()
        );
    }
}
