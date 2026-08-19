# Timeouts and Connection Lifecycle

## What problem does it solve?

Networks are unreliable. Without timeouts, a synchronous program can wait
forever for a peer that has crashed, a route that has vanished, or a packet
that was lost.

## What is happening underneath?

Rust's `std::net::TcpStream` supports:

- `set_read_timeout`
- `set_write_timeout`
- `connect_timeout`

The OS implements these with timers around socket operations. When a timeout
fires, the syscall returns `EAGAIN` / `ETIMEDOUT`, which Rust maps to
`ErrorKind::TimedOut` or `WouldBlock`.

## Timeout categories

- **Connect timeout**: how long to wait for the three-way handshake.
- **Read timeout**: how long to wait for data from the peer.
- **Write timeout**: how long to wait for the kernel to accept outgoing data.
- **Idle timeout**: how long a connection can be inactive before closing.

## Graceful shutdown

```text
Client          Server
   │               │
   │  shutdown(Write)  ← client says it will send no more data
   │──────────────>│
   │               │
   │  remaining data │
   │<──────────────│
   │               │
   │  close()      │
```

Half-closing the write side lets the peer know no more requests are coming
while still allowing the peer to finish responding.

## What can go wrong?

- Too-short timeouts drop healthy but slow peers.
- Too-long timeouts waste resources on dead peers.
- Timeouts on `accept` are not portable; our concurrent server uses a shutdown
  channel instead.

## What did our experiment demonstrate?

`crates/forge-network/src/timeout.rs` tests that connecting to an unreachable
RFC 5737 address times out quickly instead of hanging indefinitely.

## When should an engineer use this?

Every production network program needs bounded timeouts. Set them based on
measured latency distributions, not arbitrary guesses.
