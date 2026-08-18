# Benchmark: Work Queue Backpressure

## Environment

- Date: 2026-08-18T19:30:49+06:00
- Rust: rustc 1.97.1 (8bab26f4f 2026-07-14)
- Profile: debug
- OS: Linux nprime 7.1.8-arch1-3 #1 SMP PREEMPT_DYNAMIC x86_64 GNU/Linux
- CPU: 12th Gen Intel(R) Core(TM) i7-1260P (16 logical CPUs)
- Binary: `crates/forge-concurrency/examples/work_queue_demo.rs`

## Methodology

A single worker processes 20ms tasks. A producer submits 20 tasks to a queue with capacity 4 using non-blocking `try_submit`.

## Command

```bash
cargo run --example work_queue_demo -p forge-concurrency
```

## Results

```text
completed 4/4 tasks in 80.913379ms
```

The remaining 16 tasks were rejected with `ConcurrencyError::QueueFull`.

## Interpretation

With capacity 4 and one worker, the queue accepts at most 4 tasks immediately. Subsequent tasks are rejected, protecting the system from unbounded queue growth and memory pressure. This is backpressure in action.

## Limitations

- Single worker; more workers would increase throughput and reduce rejections.
- Deterministic task duration; real workloads vary.
