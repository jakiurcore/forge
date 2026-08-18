# Cache and Memory Locality

CPU caches are small, fast memories that hold recently accessed data.

## Locality

- **Temporal locality**: reusing the same data soon after accessing it.
- **Spatial locality**: accessing nearby data soon after a given access.

## Sequential vs random access

Sequential access benefits from prefetching and cache lines. Random access jumps around memory, causing more cache misses.

## Demo

```bash
cargo run --example locality_demo -p forge-memory
```

Results depend heavily on CPU, cache sizes, and workload.
