//! Educational page-table simulator.
//!
//! This simulator does not manipulate real kernel page tables. It demonstrates
//! virtual-to-physical address translation, page table lookup, and a small TLB
//! concept.

use crate::error::MemoryError;
use std::collections::HashMap;

/// A single page-table entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageTableEntry {
    /// Physical frame number.
    pub frame: u64,
    /// Whether the page is mapped.
    pub valid: bool,
}

/// A simple page table mapping virtual page numbers to physical frames.
#[derive(Debug, Default)]
pub struct PageTable {
    entries: HashMap<u64, PageTableEntry>,
}

impl PageTable {
    /// Create an empty page table.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Map a virtual page number to a physical frame.
    pub fn map(&mut self, page_number: u64, frame: u64) {
        self.entries
            .insert(page_number, PageTableEntry { frame, valid: true });
    }

    /// Unmap a virtual page number.
    pub fn unmap(&mut self, page_number: u64) {
        self.entries.remove(&page_number);
    }

    /// Look up a page-table entry.
    pub fn lookup(&self, page_number: u64) -> Option<PageTableEntry> {
        self.entries.get(&page_number).copied()
    }

    /// Translate a virtual address to a physical address.
    pub fn translate(&self, address: u64, page_size: usize) -> Result<u64, MemoryError> {
        let pn = address / page_size as u64;
        let offset = address % page_size as u64;
        match self.lookup(pn) {
            Some(entry) if entry.valid => Ok(entry.frame * page_size as u64 + offset),
            _ => Err(MemoryError::InvalidAddress(address)),
        }
    }

    /// Number of mapped pages.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether the page table is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// A tiny translation lookaside buffer simulation.
#[derive(Debug, Default)]
pub struct Tlb {
    entries: HashMap<u64, u64>,
}

impl Tlb {
    /// Create an empty TLB.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Look up a physical frame for a virtual page number.
    pub fn get(&self, page_number: u64) -> Option<u64> {
        self.entries.get(&page_number).copied()
    }

    /// Insert a translation.
    pub fn insert(&mut self, page_number: u64, frame: u64) {
        self.entries.insert(page_number, frame);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate_valid_address() {
        let mut pt = PageTable::new();
        pt.map(0, 10);
        pt.map(1, 20);
        assert_eq!(pt.translate(0, 4096).unwrap(), 10 * 4096);
        assert_eq!(pt.translate(4096, 4096).unwrap(), 20 * 4096);
        assert_eq!(pt.translate(4098, 4096).unwrap(), 20 * 4096 + 2);
    }

    #[test]
    fn translate_invalid_address() {
        let pt = PageTable::new();
        assert!(matches!(
            pt.translate(0, 4096),
            Err(MemoryError::InvalidAddress(0))
        ));
    }

    #[test]
    fn tlb_lookup() {
        let mut tlb = Tlb::new();
        tlb.insert(5, 100);
        assert_eq!(tlb.get(5), Some(100));
        assert_eq!(tlb.get(6), None);
    }
}
