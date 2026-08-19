# Latency Experiment

## Goal

Measure round-trip latency for a single TCP request/response on loopback.

## Methodology

Use the benchmark example with small payloads and a single client:

```bash
cargo run --release --example tcp_benchmark -- 127.0.0.1:0 64 10000
```

Average latency = total elapsed time / number of iterations.

## Results

See `benchmarks/networking/tcp-latency.md`.

## Interpretation

Loopback latency is dominated by kernel networking stack overhead and context
switches, not physical propagation. Results are useful for comparing
implementations on the same machine, not for predicting wide-area latency.
