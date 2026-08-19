# Framing and Serialization

## What problem does it solve?

TCP is a byte stream, not a message protocol. This assumption is wrong:

```text
send(message)  ≠  recv(message)
```

A single `send` may be split across multiple `recv` calls, and multiple `send`
calls may arrive in one `recv`. Framing turns the stream back into discrete
messages.

## What is happening underneath?

Forge uses length-prefixed framing:

```text
[ 4 bytes: payload length N (big-endian) ][ N bytes: payload ]
```

The decoder reads the header, checks that the length is sane, then waits for
`N` payload bytes before returning a complete frame.

## Decoder states

```text
buffer: []
→ need header

buffer: [0 0 0 5]
→ header complete, need 5 payload bytes

buffer: [0 0 0 5 'h' 'e' 'l' 'l']
→ partial payload, need 1 more byte

buffer: [0 0 0 5 'h' 'e' 'l' 'l' 'o' 0 0 0 3 'h' 'i']
→ two complete frames available
```

## What can go wrong?

- Partial header or payload: the decoder must wait for more bytes.
- Malicious length: a peer can claim a huge payload. We cap the maximum frame
  size (16 MiB by default).
- Multiple frames in one read: the decoder must consume them one at a time.
- Connection close mid-frame: treated as an error.

## What did our experiment demonstrate?

`crates/forge-network/src/framing.rs` has deterministic tests for every case
above. This proves that the decoder correctly handles real-world stream
fragmentation.

## When should an engineer use this?

Any TCP application that exchanges discrete messages needs framing. Common
schemes include length-prefixing, newline delimiters, fixed-size records, and
TLV (type-length-value). Length-prefixing is simple and robust for binary
protocols.
