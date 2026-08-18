//! Demonstrate the page-table simulator.

use forge_memory::page_table::PageTable;
use forge_memory::pages::page_size;

fn main() {
    let ps = page_size();
    println!("system page size: {} bytes", ps);

    let mut pt = PageTable::new();
    pt.map(0, 100);
    pt.map(1, 101);

    let va0 = 0u64;
    let va1 = ps as u64 + 4;
    println!(
        "translate 0x{:x} -> 0x{:x}",
        va0,
        pt.translate(va0, ps).unwrap()
    );
    println!(
        "translate 0x{:x} -> 0x{:x}",
        va1,
        pt.translate(va1, ps).unwrap()
    );

    match pt.translate(ps as u64 * 2, ps) {
        Ok(pa) => println!("translate 0x{:x} -> 0x{:x}", ps as u64 * 2, pa),
        Err(e) => println!("translate 0x{:x} failed: {}", ps as u64 * 2, e),
    }
}
