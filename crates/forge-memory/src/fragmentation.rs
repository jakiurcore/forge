//! Fragmentation metrics for the allocator simulator.

use crate::allocator::Block;

/// Fragmentation statistics for a memory region.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FragmentationStats {
    /// Total region size in bytes.
    pub total: usize,
    /// Bytes currently allocated.
    pub allocated: usize,
    /// Bytes in free blocks.
    pub free: usize,
    /// Number of free blocks.
    pub free_blocks: usize,
    /// Largest contiguous free block.
    pub largest_free_block: usize,
}

impl FragmentationStats {
    /// Compute stats from a list of blocks and the total region size.
    pub fn from_blocks(blocks: &[Block], total: usize) -> Self {
        let allocated = blocks.iter().filter(|b| b.allocated).map(|b| b.size).sum();
        let free_blocks: Vec<&Block> = blocks.iter().filter(|b| !b.allocated).collect();
        let free = free_blocks.iter().map(|b| b.size).sum();
        let largest_free_block = free_blocks.iter().map(|b| b.size).max().unwrap_or(0);

        Self {
            total,
            allocated,
            free,
            free_blocks: free_blocks.len(),
            largest_free_block,
        }
    }

    /// External fragmentation ratio: 1 - largest_free_block / total_free.
    /// Returns 0.0 when no free memory exists.
    pub fn external_fragmentation_ratio(&self) -> f64 {
        if self.free == 0 {
            0.0
        } else {
            1.0 - (self.largest_free_block as f64 / self.free as f64)
        }
    }

    /// Internal fragmentation estimate: free memory that is unavailable because
    /// it is trapped between allocated blocks.
    pub fn unusable_free_bytes(&self) -> usize {
        self.free - self.largest_free_block
    }
}
