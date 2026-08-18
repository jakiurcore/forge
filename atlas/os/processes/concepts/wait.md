# exit(), wait(), waitpid()

## exit()

A process terminates by calling `exit(status)`. The status is an 8-bit value returned to the parent.

## wait() and waitpid()

The parent calls `wait()` or `waitpid()` to:

- Block until a child changes state.
- Read the child's exit status.
- Remove the child from the process table.

## Zombie processes

If a parent does not call `wait()` after a child exits, the child remains a **zombie**: a process entry with no memory but a PID and exit status. Zombies are reclaimed when the parent exits or waits.

## Exit status decoding

- Normal exit: status code 0–255.
- Killed by signal: signal number reported.

Forge's `wait` module decodes these in `crates/forge-process/src/wait.rs`.
