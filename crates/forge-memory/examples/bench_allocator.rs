//! Compare allocator strategies under a mixed workload.

use forge_memory::allocator::SimulatedAllocator;
use forge_memory::fragmentation::FragmentationStats;
use forge_memory::strategy::AllocationStrategy;

fn run_workload(
    strategy: AllocationStrategy,
    region: usize,
    ops: &[(usize, bool)],
) -> FragmentationStats {
    let mut alloc = SimulatedAllocator::new(region);
    alloc.set_strategy(strategy);
    let mut ids = Vec::new();

    for &(size, should_free) in ops {
        if should_free {
            if let Some(id) = ids.pop() {
                alloc.free(id).ok();
            }
        } else if let Ok(id) = alloc.allocate(size, 1) {
            ids.push(id);
        }
    }

    FragmentationStats::from_blocks(alloc.blocks(), alloc.capacity())
}

fn main() {
    let region = 1024 * 1024;
    let mut ops = Vec::new();
    for _ in 0..500 {
        ops.push((1024, false)); // allocate
        ops.push((2048, false));
        ops.push((512, false));
        ops.push((0, true)); // free one
    }

    for strategy in [
        AllocationStrategy::FirstFit,
        AllocationStrategy::BestFit,
        AllocationStrategy::WorstFit,
    ] {
        let stats = run_workload(strategy, region, &ops);
        println!("{:?}:", strategy);
        println!("  allocated: {} bytes", stats.allocated);
        println!("  free: {} bytes", stats.free);
        println!("  free blocks: {}", stats.free_blocks);
        println!("  largest free block: {} bytes", stats.largest_free_block);
        println!(
            "  external fragmentation: {:.2}",
            stats.external_fragmentation_ratio()
        );
    }
}
