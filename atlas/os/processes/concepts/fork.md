# fork()

`fork()` creates a new process by duplicating the calling process. After `fork()`:

- The **parent** receives the child's PID.
- The **child** receives `0` and begins execution at the same point.
- Both processes have copies of the same memory, file descriptors, and code.

```text
parent
  |
 fork()
  |
  +-- parent (pid unchanged)
  +-- child  (new pid, ppid = parent)
```

## Copy-on-write

Modern kernels do not copy all memory immediately. They use **copy-on-write**: pages are shared until one process modifies them.

## Demonstration

```bash
cargo run --example fork_demo -p forge-process
```

Expected output shows the parent PID, child PID, and the child's PPID pointing back to the parent.
