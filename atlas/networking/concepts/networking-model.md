# Networking Model

## What problem does it solve?

Computer networks let independent hosts exchange data. A model helps us
separate concerns: how do bits move across a wire, how do they reach the right
host, and how do applications talk to each other?

## What is happening underneath?

A common layered view:

```text
Application
     ↓
Transport
     ↓
Internet
     ↓
Link
```

- **Link**: Ethernet, Wi-Fi, physical signals, MAC addresses.
- **Internet**: IP moves packets from host to host across networks.
- **Transport**: TCP provides a reliable byte stream; UDP provides datagrams.
- **Application**: HTTP, DNS, our Forge protocol, etc.

This is not a strict OSI seven-layer model. It is a practical engineering
view. The boundaries matter because each layer solves a different problem and
has different failure modes.

## Key concepts

- **Host**: a machine on the network.
- **Interface**: a network attachment point (e.g., `eth0`, `wlan0`).
- **IP address**: a layer-3 host identifier. IPv4 is 32 bits; IPv6 is 128 bits.
- **Port**: a 16-bit layer-4 identifier that distinguishes services on one host.
- **Loopback**: `127.0.0.1` / `::1` — traffic that never leaves the host.
- **Client/server**: one program initiates a connection (client); another
  listens and accepts (server).

## What can go wrong?

- Packets are lost, duplicated, reordered, or delayed.
- Firewalls and NAT rewrite or drop traffic.
- DNS names resolve to wrong or stale addresses.
- Binding to port 0 asks the OS for an ephemeral port; binding to a well-known
  port may require privileges.

## What did our experiment demonstrate?

`forge network inspect` lists Linux interface names from `/sys/class/net`,
showing that interfaces are kernel-managed resources and that the same host can
have multiple independent network attachments.

## When should an engineer use this?

Whenever you design a distributed system, debug connectivity, or choose
between TCP and UDP. The model tells you where to look: link layer for cables
and Wi-Fi, internet layer for routing, transport layer for reliability and
ordering, application layer for semantics.
