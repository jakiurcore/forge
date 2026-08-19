//! UDP echo example.
//!
//! Run the server:
//!
//! ```bash
//! cargo run --example udp_echo -- server 127.0.0.1:7001
//! ```
//!
//! Run the client in another terminal:
//!
//! ```bash
//! cargo run --example udp_echo -- client 127.0.0.1:7001 "hello forge"
//! ```

use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: udp_echo <server|client> <addr> [message]");
        std::process::exit(1);
    }

    let mode = &args[1];
    let addr = &args[2];

    match mode.as_str() {
        "server" => {
            let (socket, local) = forge_network::udp::bind_udp(addr)?;
            println!("udp echo server listening on {}", local);
            loop {
                let mut buf = vec![0u8; 65535];
                let (n, peer) = socket.recv_from(&mut buf)?;
                socket.send_to(&buf[..n], peer)?;
            }
        }
        "client" => {
            let message = args.get(3).map(|s| s.as_bytes()).unwrap_or(b"hello, forge");
            let (socket, _) = forge_network::udp::bind_udp("127.0.0.1:0")?;
            socket.send_to(message, addr)?;
            let mut buf = vec![0u8; 65535];
            let (n, _) = socket.recv_from(&mut buf)?;
            println!("{}", String::from_utf8_lossy(&buf[..n]));
        }
        _ => {
            eprintln!("unknown mode: {}", mode);
            std::process::exit(1);
        }
    }

    Ok(())
}
