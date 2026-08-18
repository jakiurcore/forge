# Pipe Throughput

## Environment

- Date: 2026-08-18T18:42:31+06:00
- OS: Linux nprime 7.1.8-arch1-3 #1 SMP PREEMPT_DYNAMIC x86_64 GNU/Linux
- CPU: 12th Gen Intel(R) Core(TM) i7-1260P (16 logical CPUs)
- Test binary: `crates/forge-process/examples/bench_pipe.rs`

## Methodology

Run `cargo run --example bench_pipe -p forge-process -- 4096 1000`.
The example sends 4096-byte chunks through a pipe 1000 times and measures throughput.
Each round trip creates a pipe, forks, has the child write one chunk, and has the parent read it.

## Command

```bash
cargo run --example bench_pipe -p forge-process -- 4096 1000
```

## Results

```text
chunk_size: 4096 bytes
iterations: 1000
total_bytes: 4096000
total: 92.732214ms
throughput: 42.12 MiB/s
```

## Interpretation

Pipe throughput in this microbenchmark is approximately 42 MiB/s. The number is dominated by the per-iteration fork+write+read+wait overhead rather than the kernel pipe bandwidth itself. A single long-lived pipe between two processes would achieve much higher throughput because the setup cost is amortized.
