//! Demonstrates fork() + exec() by spawning `/bin/echo` and waiting.

use forge_process::exec::fork_exec;
use forge_process::wait::wait_for_child;

fn main() {
    let child_pid = fork_exec("/bin/echo", &["hello from exec"]).expect("fork+exec failed");
    println!("Parent spawned child {}", child_pid);

    let summary = wait_for_child(child_pid).expect("wait failed");
    println!("Child {}", summary.describe());
}
