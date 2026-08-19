# UDP

## What problem does it solve?

UDP provides a minimal, connectionless datagram service. It adds port
multiplexing to IP but does not guarantee delivery, ordering, or duplication
prevention.

## What is happening underneath?

A UDP datagram contains:

```text
[ source port ][ destination port ][ length ][ checksum ][ payload ]
```

The kernel delivers each datagram as one unit (or drops it). There is no
handshake, no retransmission, and no flow control at the transport layer.

## What can go wrong?

- Datagrams can be lost, duplicated, or reordered.
- Large datagrams may be fragmented by IP and dropped if any fragment is lost.
- There is no notification that the peer received data.
- UDP does not provide congestion control; senders can overwhelm the network.

## TCP vs UDP

| Property | TCP | UDP |
|----------|-----|-----|
| Reliable | yes | no |
| Ordered | yes | no |
| Connection | yes | no |
| Message boundaries | no | yes |
| Overhead | higher | lower |
| Congestion control | yes | no |

Do not say "UDP is faster" without qualification. UDP has less overhead, but
applications must build their own reliability, ordering, and congestion control
if they need them.

## What did our experiment demonstrate?

`crates/forge-network/examples/udp_echo.rs` sends a single datagram and
receives the same payload back. It demonstrates message boundaries and the
connectionless send/receive model.

## When should an engineer use this?

Use UDP when low latency matters more than reliability, when message boundaries
are important, or when you want to implement your own reliability (e.g., QUIC,
game networking, DNS queries, VoIP).
