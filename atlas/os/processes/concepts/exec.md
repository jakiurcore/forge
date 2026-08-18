# exec()

`fork()` creates a new process. `exec()` replaces the current process image with a new program.

```text
fork()          exec("/bin/ls")
  +-- child  →  child becomes /bin/ls
```

The PID does **not** change. The memory space, code, and data are replaced by those of the new program.

## fork + exec pattern

A shell uses this pattern to run external commands:

1. `fork()` to create a child.
2. Child calls `exec()` to load the command.
3. Parent calls `wait()` to reap the child.

## Demonstration

```bash
cargo run --example exec_demo -p forge-process
```
