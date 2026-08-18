//! Demonstrates fork() and reports parent/child PIDs.

use forge_process::fork::{current_pids, demo_fork, ForkOutcome};

fn main() {
    let (pid_before, ppid_before) = current_pids();
    println!("Before fork: pid={}, ppid={}", pid_before, ppid_before);

    match demo_fork().expect("fork failed") {
        ForkOutcome::Parent { child_pid } => {
            let (pid, ppid) = current_pids();
            println!(
                "Parent after fork: pid={}, ppid={}, child_pid={}",
                pid, ppid, child_pid
            );
            // Reap the child to avoid a zombie.
            let _ = forge_process::wait::wait_for_child(child_pid);
        }
        ForkOutcome::Child => {
            let (pid, ppid) = current_pids();
            println!("Child after fork: pid={}, ppid={}", pid, ppid);
            std::process::exit(0);
        }
    }
}
