# Concurrent TCP Server

## What problem does it solve?

A single-threaded server can only handle one client at a time. A concurrent
server accepts multiple connections and dispatches each to a worker, so slow or
idle clients do not block everyone else.

## Architecture

```text
TCP Listener
      │
   accept()
      │
      ├── Connection → ThreadPool → handler
      ├── Connection → ThreadPool → handler
      └── Connection → ThreadPool → handler
```

Forge reuses the `ThreadPool` from `forge-concurrency` instead of spawning a
new OS thread for every connection. This bounds resource usage and avoids the
overhead of thread-per-connection at high concurrency.

## Trade-offs

| Approach | Pros | Cons |
|----------|------|------|
| Thread per connection | simple, blocking code | high memory, OS scheduling overhead |
| Thread pool | bounded resources, reusable | still blocks one worker per connection |
| Event loop (epoll/kqueue) | scales to many idle connections | more complex, often needs async runtime |

Our synchronous pool is an intentional educational step before async I/O.

## What can go wrong?

- Accept thread blocks until a connection arrives. We use a shutdown channel
  plus a dummy connection to stop cleanly.
- Too few workers starve clients; too many waste memory.
- A handler that panics only affects that job because the pool catches it
  inside the worker loop.

## What did our experiment demonstrate?

`crates/forge-network/src/server.rs` tests eight clients sending data to a pool
of four workers. All clients receive their echoed data, showing that the pool
concurrently handles multiple connections.

## When should an engineer use this?

Use a thread pool when you need moderate concurrency, want predictable resource
usage, and can tolerate one OS thread per active connection. Move to event-loop
or async architectures when you need to support tens of thousands of
simultaneous connections.
