# Pages

Memory is managed in fixed-size chunks called **pages** (typically 4 KiB on x86_64 Linux).

## Page size

The system page size can be queried at runtime:

```bash
forge memory page-size
```

## Page number and offset

A virtual address splits into:

```text
virtual address = page number × page size + offset
```

## Page faults

A **page fault** occurs when a program accesses a page that is not mapped or not present in physical memory. The OS handles the fault, loads the page if needed, and resumes execution.
