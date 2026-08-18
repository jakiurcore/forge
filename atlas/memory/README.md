# Memory & Memory Management

This module covers Days 21–30 of the Engineering Atlas. It explains how memory works from virtual addresses down to CPU caches, with simulators and experiments in `crates/forge-memory`.

## Learning path

1. [Memory model](concepts/memory-model.md)
2. [Virtual memory](concepts/virtual-memory.md)
3. [Pages](concepts/pages.md)
4. [Page tables](concepts/page-tables.md)
5. [Stack and heap](concepts/stack-and-heap.md)
6. [Allocators](concepts/allocators.md)
7. [Fragmentation](concepts/fragmentation.md)
8. [mmap](concepts/mmap.md)
9. [Copy-on-write](concepts/copy-on-write.md)
10. [Cache and locality](concepts/cache-locality.md)

## Runnable code

See [examples/README.md](examples/README.md) and `crates/forge-memory/examples/`.

## Measurements

- [Allocation](experiments/allocation.md)
- [Fragmentation](experiments/fragmentation.md)
- [mmap](experiments/mmap.md)
- [Copy-on-write](experiments/copy-on-write.md)
- [Cache locality](experiments/cache-locality.md)

## CLI usage

```bash
forge memory page-size
forge memory inspect <pid>
```

## Platform note

`/proc`, `mmap`, and fork/COW experiments are Linux-specific. Simulators are OS-independent.
