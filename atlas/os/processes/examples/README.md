# Process Examples

These runnable examples live in `crates/forge-process/examples/` and demonstrate core Unix process concepts.

| Example | Command | Concept |
|---|---|---|
| fork_demo | `cargo run --example fork_demo -p forge-process` | fork(), PID, PPID |
| exec_demo | `cargo run --example exec_demo -p forge-process` | fork() + exec() |
| signal_demo | `cargo run --example signal_demo -p forge-process` | signal handling |
| pipe_demo | `cargo run --example pipe_demo -p forge-process` | pipes and IPC |
| bench_fork | `cargo run --example bench_fork -p forge-process -- 1000` | fork latency |
| bench_exec | `cargo run --example bench_exec -p forge-process -- 500` | fork+exec latency |
| bench_pipe | `cargo run --example bench_pipe -p forge-process -- 4096 1000` | pipe throughput |

Each example is a small standalone program intended for learning and measurement.
