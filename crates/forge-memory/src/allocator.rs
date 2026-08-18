//! Educational fixed-region allocator simulator.
//!
//! This allocator does NOT replace the system allocator. It manages a simulated
//! memory region and is intended for learning allocator mechanics.

use crate::error::MemoryError;
use crate::strategy::AllocationStrategy;

/// Description of a memory block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block {
    /// Unique block identifier.
    pub id: usize,
    /// Start offset within the simulated region.
    pub start: usize,
    /// Size in bytes.
    pub size: usize,
    /// Whether the block is currently allocated.
    pub allocated: bool,
}

/// A simple fixed-region allocator simulator.
#[derive(Debug)]
pub struct SimulatedAllocator {
    region_size: usize,
    blocks: Vec<Block>,
    next_id: usize,
    strategy: AllocationStrategy,
}

impl SimulatedAllocator {
    /// Create a simulator for a region of `size` bytes.
    pub fn new(size: usize) -> Self {
        Self {
            region_size: size,
            blocks: vec![Block {
                id: 0,
                start: 0,
                size,
                allocated: false,
            }],
            next_id: 1,
            strategy: AllocationStrategy::FirstFit,
        }
    }

    /// Set the allocation strategy.
    pub fn set_strategy(&mut self, strategy: AllocationStrategy) {
        self.strategy = strategy;
    }

    /// Allocate `size` bytes with the given alignment.
    ///
    /// Returns a block id that can be passed to `free`.
    pub fn allocate(&mut self, size: usize, align: usize) -> Result<usize, MemoryError> {
        if size == 0 {
            return Err(MemoryError::Other(
                "allocation size must be > 0".to_string(),
            ));
        }
        if !align.is_power_of_two() {
            return Err(MemoryError::Other(
                "alignment must be a power of two".to_string(),
            ));
        }

        let candidates: Vec<&Block> = self.blocks.iter().filter(|b| !b.allocated).collect();
        let id = self
            .strategy
            .select(&candidates, size)
            .ok_or(MemoryError::OutOfMemory)?;

        let idx = self
            .blocks
            .iter()
            .position(|b| b.id == id)
            .expect("strategy returned invalid block id");

        // Read block properties before mutation.
        let block = &self.blocks[idx];
        let aligned_start = Self::align_up(block.start, align);
        let padding = aligned_start - block.start;
        let total_needed = padding + size;

        if block.size < total_needed {
            return Err(MemoryError::OutOfMemory);
        }

        let remaining = block.size - total_needed;
        let old_start = block.start;

        {
            let block = &mut self.blocks[idx];
            block.start = aligned_start;
            block.size = size;
            block.allocated = true;
        }

        if padding > 0 {
            self.blocks.push(Block {
                id: self.next_id,
                start: old_start,
                size: padding,
                allocated: false,
            });
            self.next_id += 1;
        }

        if remaining > 0 {
            self.blocks.push(Block {
                id: self.next_id,
                start: aligned_start + size,
                size: remaining,
                allocated: false,
            });
            self.next_id += 1;
        }

        self.blocks.sort_by_key(|b| b.start);
        Ok(id)
    }

    /// Free a previously allocated block.
    pub fn free(&mut self, id: usize) -> Result<(), MemoryError> {
        let block = self
            .blocks
            .iter_mut()
            .find(|b| b.id == id)
            .ok_or_else(|| MemoryError::Other("invalid allocation id".to_string()))?;
        if !block.allocated {
            return Err(MemoryError::Other("block is already free".to_string()));
        }
        block.allocated = false;
        self.coalesce();
        Ok(())
    }

    /// Total size of the simulated memory region.
    pub fn capacity(&self) -> usize {
        self.region_size
    }

    /// Return the list of blocks (useful for inspection).
    pub fn blocks(&self) -> &[Block] {
        &self.blocks
    }

    fn align_up(value: usize, align: usize) -> usize {
        value.div_ceil(align) * align
    }

    fn coalesce(&mut self) {
        self.blocks.sort_by_key(|b| b.start);
        let mut i = 0;
        while i + 1 < self.blocks.len() {
            let current = &self.blocks[i];
            let next = &self.blocks[i + 1];
            if !current.allocated && !next.allocated && current.start + current.size == next.start {
                let new_size = current.size + next.size;
                let current_id = current.id;
                self.blocks[i].size = new_size;
                self.blocks.remove(i + 1);
                // Keep the id of the earlier block.
                self.blocks[i].id = current_id;
            } else {
                i += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocate_and_free() {
        let mut alloc = SimulatedAllocator::new(1024);
        let id = alloc.allocate(100, 1).unwrap();
        let block = alloc.blocks().iter().find(|b| b.id == id).unwrap();
        assert_eq!(block.size, 100);
        assert!(block.allocated);
        alloc.free(id).unwrap();
        let block = alloc.blocks().iter().find(|b| b.id == id).unwrap();
        assert!(!block.allocated);
    }

    #[test]
    fn splitting_and_coalescing() {
        let mut alloc = SimulatedAllocator::new(1024);
        let a = alloc.allocate(100, 1).unwrap();
        let b = alloc.allocate(100, 1).unwrap();
        alloc.free(a).unwrap();
        alloc.free(b).unwrap();
        // After coalescing there should be one free block.
        assert_eq!(alloc.blocks().iter().filter(|b| !b.allocated).count(), 1);
    }

    #[test]
    fn alignment_creates_padding() {
        let mut alloc = SimulatedAllocator::new(1024);
        let id = alloc.allocate(8, 16).unwrap();
        let block = alloc.blocks().iter().find(|b| b.id == id).unwrap();
        assert_eq!(block.start % 16, 0);
        assert_eq!(block.size, 8);
    }

    #[test]
    fn out_of_memory() {
        let mut alloc = SimulatedAllocator::new(16);
        assert!(alloc.allocate(32, 1).is_err());
    }
}
