# Shared Memory

Threads in the same process share memory. This makes communication fast but requires synchronization.

## Stack vs heap

- **Stack**: per-thread, stores local variables.
- **Heap**: shared, stores dynamically allocated data.

## Ownership and aliasing

When multiple threads can access the same data, Rust's ownership rules prevent data races at compile time. Shared mutable state requires `Mutex`, `RwLock`, atomics, or other synchronization.

## Data races

A data race occurs when two threads access the same memory location concurrently, at least one is a write, and there is no synchronization. Rust's borrow checker rules out data races in safe code.

## Safe demonstration

```bash
cargo run --example shared_state -p forge-concurrency
```

This compares a deliberately racy counter with a mutex-protected counter.
