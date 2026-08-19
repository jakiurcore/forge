//! Forge Protocol v1 client example.
//!
//! Run against the protocol server:
//!
//! ```bash
//! cargo run --example protocol_client -- 127.0.0.1:7002 ping
//! cargo run --example protocol_client -- 127.0.0.1:7002 echo "hello forge"
//! cargo run --example protocol_client -- 127.0.0.1:7002 status
//! ```

use forge_network::framing::FrameCodec;
use forge_network::protocol::{Command, Request, Response, Status};
use std::env;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: protocol_client <addr> <ping|echo|status> [message]");
        std::process::exit(1);
    }

    let addr = &args[1];
    let command = &args[2];
    let payload = args
        .get(3)
        .map(|s| s.as_bytes().to_vec())
        .unwrap_or_default();

    let command = match command.as_str() {
        "ping" => Command::Ping,
        "echo" => Command::Echo,
        "status" => Command::Status,
        _ => {
            eprintln!("unknown command: {}", command);
            std::process::exit(1);
        }
    };

    let request = Request::new(command, payload)?;
    let frame = request.encode()?;

    let mut stream = TcpStream::connect(addr)?;
    stream.set_read_timeout(Some(Duration::from_secs(2)))?;
    stream.set_write_timeout(Some(Duration::from_secs(2)))?;
    stream.write_all(&frame)?;

    // Half-close so the server knows no more requests are coming.
    let _ = stream.shutdown(std::net::Shutdown::Write);

    let mut buffer = Vec::new();
    let mut chunk = [0u8; 4096];
    loop {
        match stream.read(&mut chunk) {
            Ok(0) => break,
            Ok(n) => buffer.extend_from_slice(&chunk[..n]),
            Err(e) => {
                eprintln!("read error: {}", e);
                break;
            }
        }
    }

    let codec = FrameCodec::new();
    if let Some(raw) = codec.decode(&mut buffer)? {
        let response = Response::decode(&raw)?;
        println!(
            "status: {:?}",
            match response.status {
                Status::Ok => "OK",
                Status::BadRequest => "BadRequest",
                Status::Error => "Error",
            }
        );
        println!("payload: {}", String::from_utf8_lossy(&response.payload));
    } else {
        println!("no complete response received");
    }

    Ok(())
}
