# What is a Process?

A **program** is a file containing instructions and data. A **process** is a program in execution: an instance that has been loaded into memory and given resources by the operating system.

## Key identifiers

- **PID** — Process ID. A unique number identifying the process.
- **PPID** — Parent PID. The PID of the process that created this one.
- **UID** — User ID. The user the process runs as.
- **GID** — Group ID. The primary group of the process.

## Process states

A process is in one of several states:

- **Running (R)** — executing or ready to execute.
- **Sleeping (S)** — waiting for an event, interruptible.
- **Waiting (D)** — uninterruptible disk sleep.
- **Stopped (T)** — suspended by a signal.
- **Zombie (Z)** — terminated, waiting for the parent to read its exit status.

## User space vs kernel space

- **User space** — where the process runs its own code and accesses its own memory.
- **Kernel space** — where the OS kernel runs. A process enters kernel space through **system calls**.

## Lifecycle

```text
create → ready → running → waiting/blocked → terminated
```

A process is created by `fork()`, becomes a new program via `exec()`, and ends with `exit()`. The parent typically calls `wait()` to reap it.

## Demo

```bash
cargo run --example fork_demo -p forge-process
```

This prints the PID and PPID before and after fork.
