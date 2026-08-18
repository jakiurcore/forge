# Signals

Signals are asynchronous notifications sent to a process. They interrupt normal execution and can terminate, suspend, or be handled by the process.

## Common signals

| Signal | Default action | Meaning |
|---|---|---|
| SIGTERM | Terminate | Polite request to stop. |
| SIGINT | Terminate | Interrupt (Ctrl-C). |
| SIGKILL | Terminate | Force kill; cannot be caught. |
| SIGHUP | Terminate | Hangup on controlling terminal. |
| SIGCHLD | Ignore | Child stopped or terminated. |

## Handling

A process can install a signal handler or choose to ignore certain signals. `SIGKILL` and `SIGSTOP` cannot be caught or ignored.

## Demonstration

```bash
cargo run --example signal_demo -p forge-process
```

Send `SIGTERM` or `SIGINT` from another terminal and observe that the demo ignores them for 10 seconds.
