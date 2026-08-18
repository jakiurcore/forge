//! Page size detection and address/page conversion helpers.

use libc::{sysconf, _SC_PAGESIZE};

/// Detect the system page size at runtime.
pub fn page_size() -> usize {
    // SAFETY: sysconf with _SC_PAGESIZE is always safe on Linux.
    let size = unsafe { sysconf(_SC_PAGESIZE) };
    size as usize
}

/// Return the page number for a virtual address.
pub fn page_number(address: u64, page_size: usize) -> u64 {
    address / page_size as u64
}

/// Return the offset within a page for a virtual address.
pub fn page_offset(address: u64, page_size: usize) -> u64 {
    address % page_size as u64
}

/// Round an address up to the next page boundary.
pub fn page_align_up(address: u64, page_size: usize) -> u64 {
    let ps = page_size as u64;
    address.div_ceil(ps) * ps
}

/// Round an address down to the previous page boundary.
pub fn page_align_down(address: u64, page_size: usize) -> u64 {
    address / page_size as u64 * page_size as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_size_is_reasonable() {
        let ps = page_size();
        assert!(ps.is_power_of_two());
        assert!(ps >= 4096);
    }

    #[test]
    fn page_number_and_offset() {
        let ps = 4096;
        assert_eq!(page_number(8192, ps), 2);
        assert_eq!(page_offset(8194, ps), 2);
    }

    #[test]
    fn page_alignment() {
        let ps = 4096;
        assert_eq!(page_align_up(1, ps), 4096);
        assert_eq!(page_align_up(4096, ps), 4096);
        assert_eq!(page_align_up(4097, ps), 8192);
        assert_eq!(page_align_down(4097, ps), 4096);
    }
}
