# Page Tables

Page tables store the mapping from virtual page numbers to physical frames.

```text
virtual address
      │
      ▼
page number + offset
      │
      ▼
page table
      │
      ▼
physical frame + offset
      │
      ▼
physical address
```

## Multi-level page tables

Modern CPUs use multi-level page tables to avoid storing a full map for the entire address space. Only populated regions need page-table entries.

## TLB

The **Translation Lookaside Buffer** caches recent virtual-to-physical translations. TLB misses require walking the page table, which is slower.

## Simulator

Forge provides an educational page-table simulator:

```bash
cargo run --example page_table_demo -p forge-memory
```
