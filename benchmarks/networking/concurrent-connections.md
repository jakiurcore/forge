# Concurrent Connections Benchmark

## Environment

- CPU: 12th Gen Intel(R) Core(TM) i7-1260P
- RAM: 31 GiB
- OS: Linux nprime 7.1.8-arch1-3 #1 SMP PREEMPT_DYNAMIC x86_64 GNU/Linux
- Rust: rustc 1.97.1 (8bab26f4f 2026-07-14)
- Build profile: debug (integration test)

## Methodology

- Start a `ConcurrentServer` with a 4-worker `ThreadPool`.
- Connect 8 clients sequentially, each sending a short message and reading the
  echo.
- Verify that every client receives its own message back.
- Implemented in `crates/forge-network/src/server.rs`.

## Command

```bash
cargo test -p forge-network concurrent_server_echoes_clients
```

## Results

```text
test server::tests::concurrent_server_echoes_clients ... ok
```

Elapsed: ~0.10 s (includes client setup/teardown).

## Interpretation

- A thread pool with fewer workers than clients can still serve all clients
  concurrently as long as handlers complete quickly.
- The OS accept queue buffers incoming connections until workers are
  available.
- No client blocks waiting for another because each runs in its own thread and
  each handler runs in a pool worker.

## Limitations

- Eight clients is modest; the pool would eventually saturate with many
  long-running handlers.
- Loopback eliminates real network jitter and connection-setup latency.
