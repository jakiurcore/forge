# UDP Throughput Benchmark

## Environment

- CPU: 12th Gen Intel(R) Core(TM) i7-1260P
- RAM: 31 GiB
- OS: Linux nprime 7.1.8-arch1-3 #1 SMP PREEMPT_DYNAMIC x86_64 GNU/Linux
- Rust: rustc 1.97.1 (8bab26f4f 2026-07-14)
- Build profile: release

## Methodology

- Loopback UDP using `std::net::UdpSocket`.
- Single client sends a datagram and waits for the echo; server echoes each
  datagram back.
- Throughput counts request + response bytes.
- Measured by `crates/forge-network/examples/udp_benchmark.rs`.

## Commands

```bash
cargo run --release --example udp_benchmark -- 127.0.0.1:0 1024 10000
```

## Results

| Payload | Iterations | Elapsed (s) | Datagrams/sec | Throughput (MB/s) |
|---------|------------|-------------|---------------|-------------------|
| 1024 B  | 10000      | 0.090725    | 110223.22     | 215.28            |

## Interpretation

- UDP echo throughput on loopback is comparable to small-payload TCP echo.
- There is no connection setup overhead, but the send/recv round trip still
  requires two syscalls per direction.
- The throughput number is lower than raw UDP capacity because the client
  blocks for each response before sending the next datagram.

## Limitations

- Loopback avoids packet loss, fragmentation, and reordering.
- A blocking client limits throughput; pipelining or batched sends would yield
  higher numbers at the cost of application complexity.
