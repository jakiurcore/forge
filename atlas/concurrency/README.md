# Threads & Concurrency

This module covers Days 11–20 of the Engineering Atlas. It builds reusable concurrency primitives in `crates/forge-concurrency` and explains the concepts behind them.

## Learning path

1. [Threads](concepts/threads.md)
2. [Shared memory](concepts/shared-memory.md)
3. [Mutexes](concepts/mutex.md)
4. [Read/write locks](concepts/rwlock.md)
5. [Atomics](concepts/atomics.md)
6. [Channels](concepts/channels.md)
7. [Thread pools](concepts/thread-pool.md)
8. [Work queues](concepts/work-queues.md)
9. [Races and deadlocks](concepts/deadlocks.md)

## Runnable code

See [examples/README.md](examples/README.md) and `crates/forge-concurrency/examples/`.

## Measurements

- [Mutex vs atomic](experiments/mutex-vs-atomic.md)
- [Channel throughput](experiments/channel-throughput.md)
- [Thread pool](experiments/thread-pool.md)
- [Queue backpressure](experiments/queue-backpressure.md)
- [Contention](experiments/contention.md)

## CLI usage

```bash
forge concurrency race-demo
forge concurrency deadlock-demo
```
