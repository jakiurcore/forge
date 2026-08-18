//! Demonstrates parent/child pipe communication.

use forge_process::pipe::{demo_pipe_message, pipe_exec};

fn main() {
    let message = b"hello through a pipe";
    let received = demo_pipe_message(message).expect("pipe demo failed");
    println!("Child wrote: {:?}", String::from_utf8_lossy(&received));

    let output = pipe_exec("/bin/echo", &["hello from exec pipe"]).expect("pipe exec failed");
    println!("pipe_exec captured: {:?}", String::from_utf8_lossy(&output));
}
