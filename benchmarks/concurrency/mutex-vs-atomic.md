# Benchmark: Mutex Counter vs Atomic Counter

## Environment

- Date: 2026-08-18T19:30:49+06:00
- Rust: rustc 1.97.1 (8bab26f4f 2026-07-14)
- Profile: debug
- OS: Linux nprime 7.1.8-arch1-3 #1 SMP PREEMPT_DYNAMIC x86_64 GNU/Linux
- CPU: 12th Gen Intel(R) Core(TM) i7-1260P (16 logical CPUs)
- Binary: `crates/forge-concurrency/examples/bench_mutex_vs_atomic.rs`

## Methodology

Increment a shared counter 100,000 times per thread across 8 threads. Measure wall-clock time for a mutex-protected counter and an atomic counter.

## Command

```bash
cargo run --example bench_mutex_vs_atomic -p forge-concurrency
```

## Results

```text
threads: 8
increments per thread: 100000
mutex total: 169.715082ms
atomic total: 28.789174ms
mutex per increment: 212ns
atomic per increment: 35ns
```

## Interpretation

The atomic counter is approximately 6× faster under high contention in this microbenchmark. The mutex introduces kernel scheduling and cache-line bouncing overhead that atomics avoid for simple operations.

## Limitations

- Debug build; release builds would be faster for both.
- The workload is purely contended increments. Real programs mix reads, writes, and non-shared work, changing the relative performance.
