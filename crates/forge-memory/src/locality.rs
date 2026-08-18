//! Cache and memory-locality experiments.

use std::time::{Duration, Instant};

/// Sum a slice sequentially and return the elapsed time.
pub fn bench_sequential(data: &[u64]) -> Duration {
    let start = Instant::now();
    let mut sum = 0u64;
    for &value in data {
        sum = sum.wrapping_add(value);
    }
    let elapsed = start.elapsed();
    // Prevent the sum from being optimized away.
    std::hint::black_box(sum);
    elapsed
}

/// Sum a slice in random order and return the elapsed time.
pub fn bench_random(data: &[u64], indices: &[usize]) -> Duration {
    let start = Instant::now();
    let mut sum = 0u64;
    for &idx in indices {
        sum = sum.wrapping_add(data[idx]);
    }
    let elapsed = start.elapsed();
    std::hint::black_box(sum);
    elapsed
}

/// Generate a pseudo-random permutation of indices for `len`.
///
/// Uses a simple LCG so no external random crate is needed.
pub fn random_indices(len: usize) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..len).collect();
    let mut state = 0x1234_5678_9ABC_DEF0u64;
    for i in (1..len).rev() {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let j = (state as usize) % (i + 1);
        indices.swap(i, j);
    }
    indices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequential_and_random_sum_equal() {
        let data: Vec<u64> = (0..1000).map(|x| x as u64).collect();
        let indices = random_indices(data.len());

        let mut seq_sum = 0u64;
        for &v in &data {
            seq_sum = seq_sum.wrapping_add(v);
        }

        let mut rand_sum = 0u64;
        for &idx in &indices {
            rand_sum = rand_sum.wrapping_add(data[idx]);
        }

        assert_eq!(seq_sum, rand_sum);
    }
}
