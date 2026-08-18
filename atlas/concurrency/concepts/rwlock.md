# Read/Write Locks

An `RwLock` allows many readers or one writer at a time.

## When it helps

- Read-heavy workloads benefit because reads do not block each other.
- Write-heavy workloads may perform worse than a mutex due to writer overhead.

## Trade-offs

- More complex than a mutex.
- Writers can starve if readers continuously acquire the lock.

## Implementation

Forge provides `RwLockedCache` in `crates/forge-concurrency/src/rwlocked.rs`.
