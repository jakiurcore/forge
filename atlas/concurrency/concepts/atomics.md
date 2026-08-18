# Atomics

Atomic operations are indivisible: they complete without interruption.

## Operations

- `load` / `store`
- `fetch_add` / `fetch_sub`
- `compare_and_swap`

## Memory ordering

Memory ordering controls how atomic operations are observed by other threads.

- **SeqCst**: strongest, easiest to reason about, often slightly slower.
- **Acquire/Release**: pair for producer/consumer synchronization.
- **Relaxed**: fastest, weakest guarantees.

## When to use

Use atomics for simple counters and flags. Prefer mutexes or channels for complex state.

## Implementation

Forge provides `AtomicCounter` and `AtomicFlag` in `crates/forge-concurrency/src/atomic.rs`.
