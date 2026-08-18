//! Allocation strategies for the educational allocator simulator.

use crate::allocator::Block;

/// Strategy for choosing a free block during allocation.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum AllocationStrategy {
    /// Choose the first block that fits.
    #[default]
    FirstFit,
    /// Choose the smallest block that fits.
    BestFit,
    /// Choose the largest block that fits.
    WorstFit,
}

impl AllocationStrategy {
    /// Select a free block from `candidates` for a request of `size` bytes.
    pub fn select(&self, candidates: &[&Block], size: usize) -> Option<usize> {
        match self {
            AllocationStrategy::FirstFit => candidates
                .iter()
                .position(|b| b.size >= size)
                .map(|idx| candidates[idx].id),
            AllocationStrategy::BestFit => candidates
                .iter()
                .filter(|b| b.size >= size)
                .min_by_key(|b| b.size)
                .map(|b| b.id),
            AllocationStrategy::WorstFit => candidates
                .iter()
                .filter(|b| b.size >= size)
                .max_by_key(|b| b.size)
                .map(|b| b.id),
        }
    }
}
