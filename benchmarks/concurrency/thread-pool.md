# Benchmark: Thread Pool Throughput

## Environment

- Date: 2026-08-18T19:30:49+06:00
- Rust: rustc 1.97.1 (8bab26f4f 2026-07-14)
- Profile: debug
- OS: Linux nprime 7.1.8-arch1-3 #1 SMP PREEMPT_DYNAMIC x86_64 GNU/Linux
- CPU: 12th Gen Intel(R) Core(TM) i7-1260P (16 logical CPUs)
- Binary: `crates/forge-concurrency/examples/bench_pool.rs`

## Methodology

Submit 100,000 no-op jobs to a 4-worker thread pool and measure total time to complete.

## Command

```bash
cargo run --example bench_pool -p forge-concurrency
```

## Results

```text
workers: 4
jobs: 100000
completed: 100000
elapsed: 46.432348ms
per job: 464ns
```

## Interpretation

The pool can process over 2 million trivial jobs per second in this configuration. The per-job overhead is dominated by channel send/recv and atomic counter update, not actual work.

## Limitations

- Jobs are no-ops. CPU-bound work would change throughput and scaling.
- Debug build.
