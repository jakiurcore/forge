# mmap

`mmap()` maps files or anonymous memory into a process's address space.

## Types

- **Anonymous**: no file backing; used for large allocations and shared memory.
- **File-backed**: the contents of a file are mapped directly into memory.
- **Private**: changes are not written back to the file.
- **Shared**: changes are visible to other processes mapping the same file.

## Demo

```bash
cargo run --example mmap_demo -p forge-memory
```

## When to use

Use `mmap` for file I/O without explicit `read`/`write`, large zero-filled regions, or inter-process shared memory.
