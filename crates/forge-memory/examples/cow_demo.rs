//! Demonstrate fork + copy-on-write concept.

use forge_memory::cow::fork_and_touch;

fn main() {
    let size = 4 * 1024 * 1024; // 4 MiB
    let child = fork_and_touch(size, true).unwrap();
    println!(
        "forked child {} with {} MiB shared/touched buffer",
        child,
        size / (1024 * 1024)
    );
    println!("inspect with: cat /proc/{}/smaps | head -20", child);

    // Reap the child.
    let _ = nix::sys::wait::waitpid(Some(nix::unistd::Pid::from_raw(child)), None);
}
