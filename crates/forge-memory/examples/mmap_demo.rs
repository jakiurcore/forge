//! Demonstrate anonymous and file-backed mmap.

//! Demonstrate anonymous and file-backed mmap.

use forge_memory::mmap::MmapRegion;
use std::fs::{self, File};
use std::io::Write;

fn main() {
    let mut region = MmapRegion::anonymous(4096).unwrap();
    unsafe {
        region.as_slice_mut()[0] = b'H';
        region.as_slice_mut()[1] = b'i';
        println!(
            "anonymous mapping: {:?}",
            std::str::from_utf8(&region.as_slice()[..2]).unwrap()
        );
    }

    let path = "/tmp/forge_mmap_demo.txt";
    {
        let mut f = File::create(path).unwrap();
        f.write_all(b"mapped file content").unwrap();
    }
    let file = File::open(path).unwrap();

    let file_region = MmapRegion::file(&file, 19).unwrap();
    unsafe {
        println!(
            "file mapping: {:?}",
            std::str::from_utf8(file_region.as_slice()).unwrap()
        );
    }

    let _ = fs::remove_file(path);
}
