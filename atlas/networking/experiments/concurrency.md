# Concurrent Connections Experiment

## Goal

Verify that the thread-pool-based TCP server handles multiple simultaneous
clients without one client blocking another.

## Methodology

Run the integration test in `crates/forge-network/src/server.rs`:

```bash
cargo test -p forge-network concurrent_server_echoes_clients
```

The test starts a server with four workers and sends data from eight clients.

## Results

See `benchmarks/networking/concurrent-connections.md`.

## Interpretation

With a pool smaller than the client count, clients share workers. As long as
handlers are short, the pool keeps up. Long-running handlers would require more
workers or an event-driven architecture.
