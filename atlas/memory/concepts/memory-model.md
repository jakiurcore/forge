# Memory Model

A process sees memory as a contiguous address space, but modern systems use many layers of abstraction.

## Process address space (conceptual)

```text
High Address
────────────────
Stack
        ...
Heap
────────────────
Data
────────────────
Code/Text
────────────────
Low Address
```

- **Code/Text**: program instructions.
- **Data**: global and static variables.
- **Heap**: dynamically allocated memory with explicit lifetime.
- **Stack**: local variables and call frames.

## What is observable

On Linux, `/proc/<pid>/maps` shows real mapped regions. The neat stack/heap/data split is a conceptual model; the actual layout depends on the OS, linker, and runtime.

## Demo

```bash
forge memory inspect $$
```
