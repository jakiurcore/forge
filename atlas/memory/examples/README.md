# Memory Examples

Runnable examples live in `crates/forge-memory/examples/`.

| Example | Command | Concept |
|---|---|---|
| page_table_demo | `cargo run --example page_table_demo -p forge-memory` | page-table translation |
| allocator_demo | `cargo run --example allocator_demo -p forge-memory` | allocation/free/coalescing |
| mmap_demo | `cargo run --example mmap_demo -p forge-memory` | anonymous/file mmap |
| cow_demo | `cargo run --example cow_demo -p forge-memory` | fork + COW |
| locality_demo | `cargo run --example locality_demo -p forge-memory` | sequential vs random |
| bench_allocator | `cargo run --example bench_allocator -p forge-memory` | strategy comparison |
| bench_locality | `cargo run --example bench_locality -p forge-memory` | locality benchmark |
