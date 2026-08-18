# Races, Deadlocks & Contention

## Data races

A data race is unsafe concurrent access to shared mutable state. Rust prevents them in safe code.

## Deadlocks

A deadlock occurs when threads wait on each other forever, usually because of inconsistent lock ordering.

```text
Thread A        Thread B
   │               │
   ▼               ▼
 Lock A          Lock B
   │               │
   └──────X────────┘
          DEADLOCK
```

## Avoiding deadlocks

- Always acquire locks in the same order.
- Minimize critical sections.
- Avoid nested locks.
- Prefer message passing.
- Use timeouts where appropriate.

## Demonstration

```bash
forge concurrency deadlock-demo
```

This runs a timed deadlock demonstration and reports whether it completed or timed out.
