# TCP Latency Benchmark

## Environment

- CPU: 12th Gen Intel(R) Core(TM) i7-1260P
- RAM: 31 GiB
- OS: Linux nprime 7.1.8-arch1-3 #1 SMP PREEMPT_DYNAMIC x86_64 GNU/Linux
- Rust: rustc 1.97.1 (8bab26f4f 2026-07-14)
- Build profile: release

## Methodology

- Single synchronous client on loopback.
- Average round-trip latency = total elapsed time / number of iterations.
- Measured by `crates/forge-network/examples/tcp_benchmark.rs`.

## Commands

```bash
cargo run --release --example tcp_benchmark -- 127.0.0.1:0 64 10000 1
cargo run --release --example tcp_benchmark -- 127.0.0.1:0 1024 10000 1
```

## Results

| Payload | Iterations | Avg latency |
|---------|------------|-------------|
| 64 B    | 10000      | 8297 ns     |
| 1024 B  | 10000      | 8609 ns     |

## Interpretation

- Loopback round-trip latency is in the single-digit microsecond range.
- Latency is dominated by kernel networking stack traversal and context
  switches, not by propagation delay.
- Larger payloads do not increase latency proportionally because the kernel
  copies data efficiently and loopback bandwidth is high.

## Limitations

- This is a mean over many iterations; tail latency is not measured.
- Loopback bypasses physical NIC queuing and interrupt coalescing.
- A real network would add milliseconds or more depending on distance.
