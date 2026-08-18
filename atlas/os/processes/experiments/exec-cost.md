# exec() Cost

## Environment

- Date: 2026-08-18T18:42:31+06:00
- OS: Linux nprime 7.1.8-arch1-3 #1 SMP PREEMPT_DYNAMIC x86_64 GNU/Linux
- CPU: 12th Gen Intel(R) Core(TM) i7-1260P (16 logical CPUs)
- Test binary: `crates/forge-process/examples/bench_exec.rs`

## Methodology

Run `cargo run --example bench_exec -p forge-process -- 500`.
The example forks and execs `/bin/true`, then waits for the child.
It reports total time and average time per fork+exec+wait cycle.

## Command

```bash
cargo run --example bench_exec -p forge-process -- 500
```

## Results

```text
iterations: 500
total: 236.780904ms
per fork+exec+wait: 473.561µs
```

## Interpretation

Forking plus executing a trivial program (`/bin/true`) and waiting takes roughly 474 µs per cycle. The exec step dominates the overhead beyond a plain fork because the kernel must load the new program, set up memory mappings, and resolve the interpreter/dynamic linker. This is why process-spawning servers often prefer thread pools or event loops.
