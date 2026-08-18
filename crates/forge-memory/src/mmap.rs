//! Safe mmap/munmap wrappers for Linux.

use crate::error::MemoryError;
use libc::{mmap, munmap, MAP_ANONYMOUS, MAP_PRIVATE, PROT_READ, PROT_WRITE};
use std::fs::File;
use std::os::fd::AsRawFd;
use std::ptr::null_mut;

/// A mapped memory region.
///
/// The region is unmapped when this struct is dropped.
pub struct MmapRegion {
    ptr: *mut u8,
    len: usize,
}

// SAFETY: MmapRegion owns the mapping and is only accessed through the
// provided methods, which ensure no concurrent aliasing.
unsafe impl Send for MmapRegion {}
unsafe impl Sync for MmapRegion {}

impl MmapRegion {
    /// Map `len` bytes of anonymous readable/writable memory.
    pub fn anonymous(len: usize) -> Result<Self, MemoryError> {
        // SAFETY: mmap arguments are valid; we check for MAP_FAILED.
        let ptr = unsafe {
            mmap(
                null_mut(),
                len,
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANONYMOUS,
                -1,
                0,
            )
        };
        if ptr == libc::MAP_FAILED {
            return Err(MemoryError::SystemError("mmap failed".to_string()));
        }
        Ok(Self {
            ptr: ptr as *mut u8,
            len,
        })
    }

    /// Map `len` bytes of a file as readable/writable private memory.
    pub fn file(file: &File, len: usize) -> Result<Self, MemoryError> {
        let fd = file.as_raw_fd();
        let ptr = unsafe { mmap(null_mut(), len, PROT_READ | PROT_WRITE, MAP_PRIVATE, fd, 0) };
        if ptr == libc::MAP_FAILED {
            return Err(MemoryError::SystemError("mmap failed".to_string()));
        }
        Ok(Self {
            ptr: ptr as *mut u8,
            len,
        })
    }

    /// Return a slice to the mapped memory.
    ///
    /// # Safety
    /// The caller must ensure no other references to the same memory exist and
    /// that the mapping is valid.
    pub unsafe fn as_slice(&self) -> &[u8] {
        std::slice::from_raw_parts(self.ptr, self.len)
    }

    /// Return a mutable slice to the mapped memory.
    ///
    /// # Safety
    /// The caller must ensure no other references to the same memory exist and
    /// that the mapping is valid.
    pub unsafe fn as_slice_mut(&mut self) -> &mut [u8] {
        std::slice::from_raw_parts_mut(self.ptr, self.len)
    }

    /// Length of the mapping in bytes.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether the mapping is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl Drop for MmapRegion {
    fn drop(&mut self) {
        // SAFETY: ptr and len came from a successful mmap call.
        unsafe {
            let _ = munmap(self.ptr as *mut _, self.len);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anonymous_mapping_read_write() {
        let mut region = MmapRegion::anonymous(4096).unwrap();
        // SAFETY: we have exclusive access to the mapping.
        unsafe {
            let slice = region.as_slice_mut();
            slice[0] = 42;
            slice[1] = 43;
            assert_eq!(slice[0], 42);
        }
    }
}
