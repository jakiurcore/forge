# Allocators

An allocator manages a region of memory, tracking free and allocated blocks.

## Responsibilities

- Satisfy allocation requests.
- Reclaim freed memory.
- Maintain alignment.
- Minimize fragmentation.

## Simulator

Forge's `SimulatedAllocator` manages a fixed virtual region:

```bash
cargo run --example allocator_demo -p forge-memory
```

It demonstrates allocation, splitting, alignment padding, freeing, and coalescing. It is educational and does not replace the system allocator.
