# Copy-on-Write

Copy-on-write (COW) lets parent and child processes share physical pages after `fork()` until one writes.

```text
Parent
  │
 fork()
  ├──────────────┐
  │              │
shared pages   shared pages
  │              │
write          no write
  │
private page
```

## Why it matters

COW makes `fork()` cheap: only page tables and metadata are copied, not the entire address space. Pages are duplicated only on write.

## Demo

```bash
cargo run --example cow_demo -p forge-memory
```

Then inspect `/proc/<pid>/smaps` of the child.
