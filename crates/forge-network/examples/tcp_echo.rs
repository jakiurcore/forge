//! TCP echo server and client example.
//!
//! Run the server:
//!
//! ```bash
//! cargo run --example tcp_echo -- server 127.0.0.1:7000
//! ```
//!
//! Run the client in another terminal:
//!
//! ```bash
//! cargo run --example tcp_echo -- client 127.0.0.1:7000 "hello forge"
//! ```

use std::env;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: tcp_echo <server|client> <addr> [message]");
        std::process::exit(1);
    }

    let mode = &args[1];
    let addr = &args[2];

    match mode.as_str() {
        "server" => {
            forge_network::tcp::run_echo_server(addr)?;
        }
        "client" => {
            let message = args.get(3).map(|s| s.as_bytes()).unwrap_or(b"hello, forge");
            let response = forge_network::tcp::send_message(
                addr,
                message,
                Duration::from_secs(2),
                Duration::from_secs(2),
            )?;
            println!("{}", String::from_utf8_lossy(&response));
        }
        _ => {
            eprintln!("unknown mode: {}", mode);
            std::process::exit(1);
        }
    }

    Ok(())
}
