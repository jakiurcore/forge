# Sockets

## What problem does it solve?

A socket is the OS abstraction that lets a process send and receive network
data. It hides the details of IP addresses, ports, and packet formats behind a
file-descriptor-like API.

## What is happening underneath?

When a program creates a socket, the kernel allocates a data structure
containing:

- protocol family (e.g., IPv4, IPv6)
- socket type (stream / datagram)
- local and remote addresses
- send and receive buffers
- connection state (for TCP)

## TCP server lifecycle

```text
socket()
   ↓
bind()
   ↓
listen()
   ↓
accept() → returns a new connected socket
   ↓
read() / write()
   ↓
close()
```

## TCP client lifecycle

```text
socket()
   ↓
connect()
   ↓
write() / read()
   ↓
close()
```

## UDP lifecycle

UDP is connectionless:

```text
socket()
   ↓
bind()          ← optional for a sender
   ↓
sendto() / recvfrom()
   ↓
close()
```

## What can go wrong?

- `bind` fails if the address is in use (`TIME_WAIT`, another process).
- `connect` fails if the host is unreachable or the port is closed.
- `accept` blocks until a client arrives unless the socket is non-blocking.
- `read` returns 0 when the peer closes the connection.

## What did our experiment demonstrate?

`crates/forge-network/examples/tcp_echo.rs` binds a listener, accepts one
connection, and echoes bytes back. This shows the basic server lifecycle without
any framing or concurrency.

## When should an engineer use this?

Use raw sockets when you need direct control over TCP/UDP behavior, when you
are building a custom protocol, or when you are learning how the OS network
stack works. Most applications should use a higher-level library or framework
once they understand the primitives.
