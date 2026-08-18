# Channels

Channels transfer ownership of messages between threads. They are a form of message passing.

```text
Producer
   │
   ▼
Channel
   │
   ├── Worker
   ├── Worker
   └── Worker
```

## Types

- **Unbounded**: sender never blocks, can grow without limit.
- **Bounded**: sender blocks when full, providing backpressure.

## Ownership

Sending a value moves ownership to the receiver. This eliminates many data-race risks.

## Implementation

Forge provides `run_producer_consumer` in `crates/forge-concurrency/src/channel.rs`.

```bash
cargo run --example producer_consumer -p forge-concurrency
```
