# Threads

A thread is the smallest unit of execution that a scheduler can manage. A process can contain many threads that share the same address space.

```text
Process
  ├── Thread
  ├── Thread
  └── Thread
```

## Process vs thread

- **Process**: owns memory, file descriptors, and resources; isolated from other processes.
- **Thread**: shares memory and file descriptors with other threads in the same process; has its own stack and register state.

## Thread lifecycle

Threads are created, run, and are eventually joined or detached.

- **spawn**: create a new thread.
- **join**: wait for a thread to finish and collect its result.
- **detach**: let the thread run independently until it exits.

## Stack

Each thread has its own stack for local variables and function calls. The heap is shared.

## Demo

```bash
cargo run --example thread_demo -p forge-concurrency
```
