# Benchmark: Allocator Strategies

## Environment

- Date: 2026-08-18T19:50:28+06:00
- Rust: rustc 1.97.1 (8bab26f4f 2026-07-14)
- Profile: debug
- OS: Linux nprime 7.1.8-arch1-3 #1 SMP PREEMPT_DYNAMIC x86_64 GNU/Linux
- CPU: 12th Gen Intel(R) Core(TM) i7-1260P (16 logical CPUs)
- Binary: `crates/forge-memory/examples/bench_allocator.rs`

## Methodology

A 1 MiB region is managed by the simulated allocator. The workload alternates allocations of 1024, 2048, and 512 bytes with occasional frees. After the workload, fragmentation is measured for FirstFit, BestFit, and WorstFit strategies.

## Command

```bash
cargo run --example bench_allocator -p forge-memory
```

## Results

```text
FirstFit:
  allocated: 1047552 bytes
  free: 1024 bytes
  free blocks: 1
  largest free block: 1024 bytes
  external fragmentation: 0.00
BestFit:
  allocated: 1047552 bytes
  free: 1024 bytes
  free blocks: 1
  largest free block: 1024 bytes
  external fragmentation: 0.00
WorstFit:
  allocated: 1047552 bytes
  free: 1024 bytes
  free blocks: 1
  largest free block: 1024 bytes
  external fragmentation: 0.00
```

## Interpretation

In this specific workload, all three strategies ended with the same fragmentation because the free/allocate pattern allowed coalescing to keep the region compact. The difference between strategies becomes visible with irregular sizes and interleaved allocations that prevent full coalescing.

## Limitations

- Debug build.
- Workload is regular; real programs produce more varied fragmentation.
