//! Memory management experiments and inspection for Forge.
//!
//! This crate provides educational simulators (page tables, allocators),
//! Linux-specific memory mapping experiments, cache/locality benchmarks, and
//! process memory inspection via `/proc`.

#![deny(missing_docs)]

pub mod allocator;
pub mod cow;
pub mod error;
pub mod fragmentation;
pub mod info;
pub mod locality;
pub mod mmap;
pub mod page_table;
pub mod pages;
pub mod strategy;

/// Re-export commonly used types.
pub use allocator::{Block, SimulatedAllocator};
pub use fragmentation::FragmentationStats;
pub use info::{parse_maps, read_maps, MemoryMapping};
pub use locality::{bench_random, bench_sequential, random_indices};
pub use mmap::MmapRegion;
pub use page_table::{PageTable, PageTableEntry, Tlb};
pub use pages::{page_align_down, page_align_up, page_number, page_offset, page_size};
pub use strategy::AllocationStrategy;
