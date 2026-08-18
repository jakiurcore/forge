# Work Queues

A bounded work queue adds capacity limits and backpressure to a thread pool.

## Key concepts

- **Capacity**: maximum number of pending tasks.
- **Backpressure**: producers slow down or are rejected when the queue is full.
- **Worker utilization**: workers pull tasks as they become available.

## Trade-offs

- Too small: frequent rejection or blocking.
- Too large: high memory use and latency under overload.

## Implementation

Forge provides `BoundedWorkQueue` in `crates/forge-concurrency/src/queue.rs`.

```bash
cargo run --example work_queue_demo -p forge-concurrency
```
