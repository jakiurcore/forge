# Mutexes

A mutex enforces mutual exclusion: only one thread can hold the lock at a time.

```text
N threads
   ↓
shared counter
   ↓
mutex
   ↓
correct result
```

## Critical section

The code between lock acquisition and release is the critical section. Keep it small to reduce contention.

## Trade-offs

- Simple and correct.
- Can become a bottleneck under high contention.
- Lock ordering matters when multiple locks are held.

## Implementation

Forge provides `SharedCounter` in `crates/forge-concurrency/src/counter.rs`.

```bash
cargo run --example shared_state -p forge-concurrency
```
