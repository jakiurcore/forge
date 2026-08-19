# Networking Fundamentals

This module covers Days 31–40 of the Engineering Atlas. It builds a practical
understanding of sockets, TCP, UDP, framing, timeouts, concurrent servers, and
small application protocols. Reusable code lives in `crates/forge-network`.

## Learning path

1. [Networking model](concepts/networking-model.md)
2. [Sockets](concepts/sockets.md)
3. [TCP](concepts/tcp.md)
4. [UDP](concepts/udp.md)
5. [Framing and serialization](concepts/framing.md)
6. [Timeouts and connection lifecycle](concepts/timeouts.md)
7. [Concurrent TCP server](concepts/concurrency.md)
8. [Application protocols](concepts/protocols.md)

## Runnable code

See [examples/README.md](examples/README.md) and `crates/forge-network/examples/`.

## Measurements

- [TCP throughput](experiments/tcp.md)
- [UDP throughput](experiments/udp.md)
- [Framing behavior](experiments/framing.md)
- [Concurrent connections](experiments/concurrency.md)
- [Latency](experiments/latency.md)

## CLI usage

```bash
forge network tcp-echo --bind 127.0.0.1:7000
forge network tcp-connect 127.0.0.1:7000 --message "hello"
forge network udp-echo --bind 127.0.0.1:7001
forge network inspect
```

## Platform note

The socket and protocol code is cross-platform Rust. Interface inspection is
Linux-specific because `std` does not expose network interfaces portably.
