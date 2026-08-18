# Benchmark: Channel Throughput

## Environment

- Date: 2026-08-18T19:30:49+06:00
- Rust: rustc 1.97.1 (8bab26f4f 2026-07-14)
- Profile: debug
- OS: Linux nprime 7.1.8-arch1-3 #1 SMP PREEMPT_DYNAMIC x86_64 GNU/Linux
- CPU: 12th Gen Intel(R) Core(TM) i7-1260P (16 logical CPUs)
- Binary: `crates/forge-concurrency/examples/bench_channel.rs`

## Methodology

Four producers send 100,000 `usize` messages each through `std::sync::mpsc`. A single consumer receives and counts them.

## Command

```bash
cargo run --example bench_channel -p forge-concurrency
```

## Results

```text
producers: 4
messages per producer: 100000
total received: 400000
elapsed: 43.626302ms
per message: 109ns
```

## Interpretation

Channels can move hundreds of thousands of small messages per second with a single consumer. Throughput is limited by the single consumer lock and message allocation/copy overhead.

## Limitations

- Single consumer; multiple consumers would require a different design.
- Debug build; release would improve numbers.
