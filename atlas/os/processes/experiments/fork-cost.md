# fork() Cost

## Environment

- Date: 2026-08-18T18:42:31+06:00
- OS: Linux nprime 7.1.8-arch1-3 #1 SMP PREEMPT_DYNAMIC x86_64 GNU/Linux
- CPU: 12th Gen Intel(R) Core(TM) i7-1260P (16 logical CPUs)
- Test binary: `crates/forge-process/examples/bench_fork.rs`

## Methodology

Run `cargo run --example bench_fork -p forge-process -- 1000`.
The example forks a child, the child exits immediately, and the parent waits with `waitpid()`.
It reports total time and average time per fork+wait pair.

## Command

```bash
cargo run --example bench_fork -p forge-process -- 1000
```

## Results

```text
iterations: 1000
total: 120.547163ms
per fork+wait: 120.547µs
```

## Interpretation

Creating a child process via `fork()` and reaping it with `wait()` takes roughly 120 µs on this machine. This includes the kernel's copy-on-write setup, scheduler overhead, and the wait syscall. Real shells and servers fork far less frequently today, but the cost remains important for high-throughput process-spawning workloads.
