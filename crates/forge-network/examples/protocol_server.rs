//! Forge Protocol v1 server example.
//!
//! Run:
//!
//! ```bash
//! cargo run --example protocol_server -- 127.0.0.1:7002
//! ```

use forge_network::protocol::{Command, Response, Status};
use forge_network::server::ConcurrentServer;
use std::env;
use std::net::TcpStream;
use std::time::Duration;

fn handle_client(mut stream: TcpStream) {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(5)));

    let mut buffer = Vec::new();
    loop {
        match forge_network::protocol::read_request(&mut stream, &mut buffer) {
            Ok(req) => {
                let response = match req.command {
                    Command::Ping => Response::new(Status::Ok, b"pong".to_vec()),
                    Command::Echo => Response::new(Status::Ok, req.payload),
                    Command::Status => {
                        Response::new(Status::Ok, b"Forge Protocol v1 server ready".to_vec())
                    }
                };
                let response = response.unwrap_or_else(|e| {
                    Response::new(Status::Error, e.to_string().into_bytes()).unwrap()
                });
                if let Err(e) = forge_network::protocol::write_response(&mut stream, &response) {
                    eprintln!("write error: {}", e);
                    break;
                }
            }
            Err(forge_network::error::NetworkError::Io(msg)) if msg == "connection closed" => {
                // Client closed the connection after its requests; not an error.
                break;
            }
            Err(e) => {
                eprintln!("request error: {}", e);
                break;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:7002".to_string());

    let server = ConcurrentServer::bind(&addr, 4)?;
    println!(
        "Forge protocol server listening on {}",
        server.local_addr()?
    );

    let _handle = server.run(handle_client);

    // Block until Ctrl-C. The server handle stays alive for the process lifetime.
    println!("Press Ctrl-C to stop.");
    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}
