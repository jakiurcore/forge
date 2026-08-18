# Fragmentation

## Internal fragmentation

Wasted space inside an allocated block because the request is smaller than the block given.

## External fragmentation

Free memory is split into many small blocks, so a large allocation cannot be satisfied even though total free memory is sufficient.

## Allocation strategies

- **First-fit**: fast, may leave small fragments at the start.
- **Best-fit**: minimizes leftover fragment, but creates many tiny fragments.
- **Worst-fit**: leaves large free blocks, but can perform poorly in practice.

## Measurement

Forge's `FragmentationStats` reports allocated/free bytes, free block count, and largest free block.
