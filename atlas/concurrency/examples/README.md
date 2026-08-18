# Concurrency Examples

Runnable examples live in `crates/forge-concurrency/examples/`.

| Example | Command | Concept |
|---|---|---|
| thread_demo | `cargo run --example thread_demo -p forge-concurrency` | spawn/join threads |
| shared_state | `cargo run --example shared_state -p forge-concurrency` | race vs mutex |
| producer_consumer | `cargo run --example producer_consumer -p forge-concurrency` | channels |
| thread_pool_demo | `cargo run --example thread_pool_demo -p forge-concurrency` | thread pool |
| work_queue_demo | `cargo run --example work_queue_demo -p forge-concurrency` | backpressure |
| bench_mutex_vs_atomic | `cargo run --example bench_mutex_vs_atomic -p forge-concurrency` | counter benchmark |
| bench_channel | `cargo run --example bench_channel -p forge-concurrency` | channel benchmark |
| bench_pool | `cargo run --example bench_pool -p forge-concurrency` | pool benchmark |
| bench_rwlock | `cargo run --example bench_rwlock -p forge-concurrency` | RwLock benchmark |
