# Benchmark: Cache Locality

## Environment

- Date: 2026-08-18T19:50:28+06:00
- Rust: rustc 1.97.1 (8bab26f4f 2026-07-14)
- Profile: debug
- OS: Linux nprime 7.1.8-arch1-3 #1 SMP PREEMPT_DYNAMIC x86_64 GNU/Linux
- CPU: 12th Gen Intel(R) Core(TM) i7-1260P (16 logical CPUs)
- Binary: `crates/forge-memory/examples/bench_locality.rs`

## Methodology

Sum the same array of `u64` values either sequentially or through a pseudo-random permutation of indices. Measure wall-clock time for three array sizes.

## Command

```bash
cargo run --example bench_locality -p forge-memory
```

## Results

```text
len: 100000
  sequential: 580.716µs
  random: 807.038µs
  ratio: 1.39
len: 1000000
  sequential: 6.246058ms
  random: 15.303925ms
  ratio: 2.45
len: 1000000
  sequential: 56.971227ms
  random: 302.541264ms
  ratio: 5.31
```

## Interpretation

As the working set grows, random access becomes much slower relative to sequential access because cache lines and prefetchers are less effective. At 10 million elements the random traversal is over 5× slower on this machine.

## Limitations

- Debug build; release would reduce absolute times but likely preserve the ratio.
- Single-threaded; multi-threaded access introduces cache coherency effects.
- CPU-specific; different cache sizes would change the crossover points.
