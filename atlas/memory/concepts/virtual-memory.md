# Virtual Memory

Virtual memory gives each process the illusion of a large, private address space while the OS maps virtual addresses to physical frames.

## Benefits

- **Isolation**: processes cannot access each other's memory.
- **Sharing**: the same physical page can be mapped into multiple processes.
- **Swapping**: rarely used pages can be moved to disk.
- **Sparse address spaces**: not every virtual address needs physical backing.

## Virtual vs physical

- **Virtual address**: what the program uses.
- **Physical address**: where data actually lives in RAM.

The CPU and OS translate virtual addresses to physical addresses on every memory access.
