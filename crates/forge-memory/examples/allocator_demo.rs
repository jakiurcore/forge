//! Demonstrate the allocator simulator.

use forge_memory::allocator::SimulatedAllocator;
use forge_memory::fragmentation::FragmentationStats;
use forge_memory::strategy::AllocationStrategy;

fn main() {
    let mut alloc = SimulatedAllocator::new(1024);

    println!("initial blocks:");
    for block in alloc.blocks() {
        println!("  {:?}", block);
    }

    let _a = alloc.allocate(100, 1).unwrap();
    let b = alloc.allocate(100, 1).unwrap();
    let _c = alloc.allocate(100, 1).unwrap();

    println!("\nafter allocations:");
    for block in alloc.blocks() {
        println!("  {:?}", block);
    }

    alloc.free(b).unwrap();
    println!("\nafter freeing B:");
    for block in alloc.blocks() {
        println!("  {:?}", block);
    }

    let _d = alloc.allocate(50, 1).unwrap();
    println!("\nafter allocating D (50 bytes):");
    for block in alloc.blocks() {
        println!("  {:?}", block);
    }

    let stats = FragmentationStats::from_blocks(alloc.blocks(), alloc.capacity());
    println!("\nfragmentation: {:?}", stats);

    // Compare strategies.
    println!("\n--- strategy comparison ---");
    for strategy in [
        AllocationStrategy::FirstFit,
        AllocationStrategy::BestFit,
        AllocationStrategy::WorstFit,
    ] {
        let mut alloc = SimulatedAllocator::new(1024);
        alloc.set_strategy(strategy);
        let _ = alloc.allocate(200, 1).unwrap();
        let _ = alloc.allocate(50, 1).unwrap();
        let _ = alloc.allocate(100, 1).unwrap();
        let stats = FragmentationStats::from_blocks(alloc.blocks(), alloc.capacity());
        println!(
            "{:?}: largest_free_block={}",
            strategy, stats.largest_free_block
        );
    }
}
