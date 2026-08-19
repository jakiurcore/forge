# TCP

## What problem does it solve?

TCP provides a **reliable, ordered, byte-stream** abstraction between two
hosts. Applications can write bytes without worrying about packet loss,
duplication, reordering, or retransmission.

## What is happening underneath?

TCP breaks the byte stream into segments, numbers them with sequence numbers,
and acknowledges receipt. It handles:

- connection establishment (three-way handshake)
- retransmission of lost segments
- in-order delivery
- flow control (receiver tells sender how much it can buffer)
- congestion control (sender adjusts rate based on network conditions)
- connection teardown (four-way close, `TIME_WAIT`)

## Connection lifecycle

```text
Client          Server
   │               │
   │    SYN        │
   │──────────────>│
   │    SYN-ACK    │
   │<──────────────│
   │    ACK        │
   │──────────────>│
   │               │
   │  data flow    │
   │<────────────> │
   │               │
   │    FIN        │
   │──────────────>│
   │    ACK        │
   │<──────────────│
   │    FIN        │
   │<──────────────│
   │    ACK        │
   │──────────────>│
```

## What can go wrong?

- A slow or vanished peer can block `read` forever without timeouts.
- Half-open connections occur when one side crashes without sending `FIN`.
- `TIME_WAIT` keeps a port busy after close.
- TCP is a stream: message boundaries are not preserved.

## What did our experiment demonstrate?

The TCP echo server and client in `crates/forge-network` show that bytes sent
are received in order. They also show that a simple `read`/`write` loop treats
the connection as a stream, which motivates framing on Day 35.

## When should an engineer use this?

Choose TCP when you need reliable, ordered delivery and can tolerate some
latency overhead. File transfer, HTTP, RPC, and database protocols usually use
TCP.
