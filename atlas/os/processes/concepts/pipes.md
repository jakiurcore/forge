# Pipes

A pipe is a unidirectional byte stream between two processes. The kernel maintains an in-memory buffer.

```text
[producer] → pipe → [consumer]
```

## pipe()

`pipe()` returns two file descriptors:

- read end
- write end

## Shell pipelines

A shell pipeline like `ls | wc` is implemented by:

1. Creating a pipe.
2. Forking twice.
3. `ls` writes to the pipe's write end (redirecting stdout).
4. `wc` reads from the pipe's read end (redirecting stdin).

## Demonstration

```bash
cargo run --example pipe_demo -p forge-process
```
