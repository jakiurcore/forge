# Processes & the Unix Process Model

This module covers Days 1–10 of the Engineering Atlas. It builds a practical, working understanding of the Unix/Linux process model and ends with a usable process inspector.

## Learning path

1. [What is a process?](concepts/process.md)
2. [Linux `/proc`](concepts/proc.md)
3. [fork()](concepts/fork.md)
4. [exec()](concepts/exec.md)
5. [exit(), wait(), waitpid()](concepts/wait.md)
6. [Signals](concepts/signals.md)
7. [Pipes](concepts/pipes.md)
8. [File descriptors](concepts/file-descriptors.md)

## Runnable code

See [examples/README.md](examples/README.md) and the source programs in `crates/forge-process/examples/`.

## Measurements

- [fork() cost](experiments/fork-cost.md)
- [exec() cost](experiments/exec-cost.md)
- [pipe throughput](experiments/pipe-throughput.md)

## CLI usage

```bash
forge process inspect <pid>
forge process fds <pid>
```

## Platform note

This module is Linux-focused. `/proc`, `fork()`, `exec()`, `waitpid()`, signals, and Unix file descriptors are POSIX/Linux-specific concepts.
