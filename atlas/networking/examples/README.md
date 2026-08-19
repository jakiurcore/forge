# Networking Examples

These examples live in `crates/forge-network/examples/`.

## TCP echo

```bash
# Terminal 1
cargo run --example tcp_echo -- server 127.0.0.1:7000

# Terminal 2
cargo run --example tcp_echo -- client 127.0.0.1:7000 "hello, forge"
```

## UDP echo

```bash
# Terminal 1
cargo run --example udp_echo -- server 127.0.0.1:7001

# Terminal 2
cargo run --example udp_echo -- client 127.0.0.1:7001 "hello, forge"
```

## Forge Protocol v1

```bash
# Terminal 1
cargo run --example protocol_server -- 127.0.0.1:7002

# Terminal 2
cargo run --example protocol_client -- 127.0.0.1:7002 ping
cargo run --example protocol_client -- 127.0.0.1:7002 echo "hello, forge"
cargo run --example protocol_client -- 127.0.0.1:7002 status
```

## TCP benchmark

```bash
cargo run --release --example tcp_benchmark -- 127.0.0.1:7003 1024 10000
```

See `benchmarks/networking/` for measured results and methodology.
