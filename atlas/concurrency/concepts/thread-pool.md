# Thread Pool

A thread pool maintains a fixed set of worker threads that execute submitted jobs.

```text
Jobs
  ↓
Queue
  ↓
ThreadPool
  ├── Worker
  ├── Worker
  ├── Worker
  └── Worker
```

## Benefits

- Avoids the cost of spawning a thread per task.
- Bounds resource usage.
- Simplifies task submission.

## Shutdown

A well-designed pool supports graceful shutdown: stop accepting jobs, finish in-flight work, and join workers.

## Implementation

Forge provides `ThreadPool` in `crates/forge-concurrency/src/pool.rs`.

```bash
cargo run --example thread_pool_demo -p forge-concurrency
```
