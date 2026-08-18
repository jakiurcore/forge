# Stack and Heap

## Stack

- Automatic storage for local variables.
- Grows and shrinks with function calls.
- Fast allocation/deallocation (just move the stack pointer).
- Limited size.

## Heap

- Dynamic storage with explicit or managed lifetime.
- Larger than the stack.
- Slower because it requires allocator bookkeeping.
- Used for values whose size is unknown at compile time or whose lifetime exceeds a single call frame.

## Trade-off

"Stack vs heap" is about lifetime and ownership, not simply speed. Large or long-lived values belong on the heap; small, temporary values belong on the stack.
