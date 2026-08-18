# Benchmark: RwLock Contention

## Environment

- Date: 2026-08-18T19:30:49+06:00
- Rust: rustc 1.97.1 (8bab26f4f 2026-07-14)
- Profile: debug
- OS: Linux nprime 7.1.8-arch1-3 #1 SMP PREEMPT_DYNAMIC x86_64 GNU/Linux
- CPU: 12th Gen Intel(R) Core(TM) i7-1260P (16 logical CPUs)
- Binary: `crates/forge-concurrency/examples/bench_rwlock.rs`

## Methodology

Two workloads:
- Read-heavy: 8 threads each perform 100,000 reads of the same key.
- Write-heavy: 8 threads each perform 10,000 inserts with unique keys.

## Command

```bash
cargo run --example bench_rwlock -p forge-concurrency
```

## Results

```text
read-heavy (8 readers x 100k): 131.139761ms
write-heavy (8 writers x 10k): 76.995332ms
```

## Interpretation

In this microbenchmark the write-heavy workload was faster than the read-heavy workload because the write volume (80,000 inserts) was lower than the read volume (800,000 reads), and the cache remained small. RwLock's read scalability becomes visible only when reads far outnumber writes and contention is sustained.

## Limitations

- Very small dataset; cache effects dominate.
- No writer starvation measurement.
- Debug build.
