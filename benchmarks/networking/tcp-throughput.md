# TCP Throughput Benchmark

## Environment

- CPU: 12th Gen Intel(R) Core(TM) i7-1260P
- RAM: 31 GiB
- OS: Linux nprime 7.1.8-arch1-3 #1 SMP PREEMPT_DYNAMIC x86_64 GNU/Linux
- Rust: rustc 1.97.1 (8bab26f4f 2026-07-14)
- Build profile: release

## Methodology

- Loopback TCP using `std::net`.
- Synchronous request/response: send payload, read full response, repeat.
- Server uses `forge-concurrency::ThreadPool` with 4 workers.
- Measured by `crates/forge-network/examples/tcp_benchmark.rs`.

## Commands

```bash
cargo run --release --example tcp_benchmark -- 127.0.0.1:0 64 10000 1
cargo run --release --example tcp_benchmark -- 127.0.0.1:0 1024 10000 1
cargo run --release --example tcp_benchmark -- 127.0.0.1:0 8192 10000 1
cargo run --release --example tcp_benchmark -- 127.0.0.1:0 1024 10000 8
```

## Results

| Payload | Clients | Iterations | Elapsed (s) | Requests/sec | Throughput (MB/s) |
|---------|---------|------------|-------------|--------------|-------------------|
| 64      | 1       | 10000      | 0.082972    | 120522.11    | 14.71             |
| 1024    | 1       | 10000      | 0.086095    | 116151.38    | 226.86            |
| 8192    | 1       | 10000      | 0.104840    | 95383.44     | 1490.37           |
| 1024    | 8       | 10000      | 0.033439    | 299049.65    | 584.08            |

## Interpretation

- Throughput rises with payload size because each round trip moves more bytes
  for a similar fixed overhead.
- Requests per second is roughly flat for small payloads because it is
  dominated by syscall and context-switch overhead, not by byte copying.
- Adding clients increases aggregate throughput because multiple connections
  keep the CPU pipeline and network buffers busier.

## Limitations

- Loopback avoids real network latency, loss, and NIC/driver effects.
- Synchronous I/O limits scalability beyond a few thousand concurrent
  connections.
- Results are specific to this machine and workload; they should not be
  generalized to other environments.
