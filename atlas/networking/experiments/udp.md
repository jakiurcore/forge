# UDP Experiment

## Goal

Observe UDP datagram behavior: message boundaries, single-packet round trips,
and the absence of connection setup.

## Methodology

- Use `crates/forge-network/examples/udp_echo.rs`.
- Bind server to an ephemeral port on loopback.
- Send a datagram and wait for the echo.

## Commands

```bash
# Terminal 1
cargo run --example udp_echo -- server 127.0.0.1:0

# Terminal 2
cargo run --example udp_echo -- client <server_addr> "hello, forge"
```

## Results

See `benchmarks/networking/udp-throughput.md` for measured throughput.

## Interpretation

UDP echoes demonstrate that each `sendto` produces one `recvfrom` and that the
kernel preserves message boundaries. Because there is no handshake, the first
round trip is as fast as subsequent ones on loopback. On a real network, loss
and jitter would appear.
