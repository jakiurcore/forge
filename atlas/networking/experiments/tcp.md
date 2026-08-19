# TCP Experiment

## Goal

Measure basic TCP request/response latency and throughput on loopback using the
synchronous stack.

## Methodology

- Use `crates/forge-network/examples/tcp_benchmark.rs`.
- Bind to `127.0.0.1` and use an ephemeral port.
- Send a request, wait for the full response, repeat.
- Record total time and bytes transferred.

## Commands

```bash
cargo run --release --example tcp_benchmark -- 127.0.0.1:0 64 10000
cargo run --release --example tcp_benchmark -- 127.0.0.1:0 1024 10000
cargo run --release --example tcp_benchmark -- 127.0.0.1:0 8192 10000
```

## Results

See `benchmarks/networking/tcp-throughput.md` and `tcp-latency.md` for the
actual measurements.

## Interpretation

Loopback TCP avoids physical network variability, so results are dominated by
system-call overhead, context switches, and buffer copies. Larger payloads
increase throughput (more bytes per syscall) but do not reduce per-round-trip
latency proportionally.
