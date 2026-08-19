# Application Protocols

## What problem does it solve?

TCP gives us a byte stream; framing gives us messages. An application protocol
adds semantics: what commands exist, what responses mean, and how to handle
errors.

## Forge Protocol v1

A minimal request/response protocol on top of length-prefixed framing.

Request:

```text
[ length: u32 BE ][ command: u8 ][ payload: bytes ]
```

Response:

```text
[ length: u32 BE ][ status: u8 ][ payload: bytes ]
```

Commands:

- `0x01` Ping
- `0x02` Echo
- `0x03` Status

Status codes:

- `0x00` Ok
- `0x01` BadRequest
- `0x02` Error

## Why this design?

- Length-prefixing handles TCP stream semantics.
- A single command byte keeps parsing trivial.
- Status codes separate successful and failed requests.
- Maximum payload size (1 MiB) prevents unbounded allocation from peers.

## What can go wrong?

- Unknown command byte → `BadRequest`.
- Payload too large → reject before allocating.
- Partial frame → wait; connection close mid-frame → error.
- A malicious client can open many connections; the server uses a bounded pool
  to limit impact.

## What did our experiment demonstrate?

`crates/forge-network/examples/protocol_server.rs` and
`protocol_client.rs` implement the full request/response flow. The client sends
`ping`, `echo`, or `status` and prints the server's reply.

## When should an engineer use this?

Real protocols (HTTP, gRPC, Redis protocol, PostgreSQL wire protocol) are more
complex, but the principles are identical: framing, commands, payloads,
statuses, length limits, and error handling. Understanding a tiny protocol
makes large protocols less mysterious.
