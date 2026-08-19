//! TCP latency and throughput benchmark.
//!
//! Run:
//!
//! ```bash
//! cargo run --release --example tcp_benchmark -- 127.0.0.1:7003 1024 10000
//! ```
//!
//! Arguments:
//!   <addr>       server address
//!   <payload>    payload size in bytes
//!   <iterations> number of request/response rounds

use forge_network::server::ConcurrentServer;
use std::env;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::thread;
use std::time::{Duration, Instant};

fn echo_handler(mut stream: TcpStream) {
    let mut buf = [0u8; 65536];
    loop {
        match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                if stream.write_all(&buf[..n]).is_err() {
                    break;
                }
            }
            Err(_) => break,
        }
    }
}

fn run_client(
    addr: &str,
    payload_size: usize,
    iterations: usize,
) -> Result<(Duration, u64), Box<dyn std::error::Error + Send + Sync>> {
    let payload = vec![0xABu8; payload_size];
    let mut stream = TcpStream::connect(addr)?;
    stream.set_read_timeout(Some(Duration::from_secs(10)))?;
    stream.set_write_timeout(Some(Duration::from_secs(10)))?;

    let start = Instant::now();
    let mut total_bytes = 0u64;

    for _ in 0..iterations {
        stream.write_all(&payload)?;
        stream.flush()?;
        total_bytes += payload.len() as u64;

        let mut received = 0usize;
        let mut buf = [0u8; 65536];
        while received < payload_size {
            let n = stream.read(&mut buf)?;
            if n == 0 {
                return Err("connection closed prematurely".into());
            }
            received += n;
        }
        total_bytes += received as u64;
    }

    let elapsed = start.elapsed();
    let _ = stream.shutdown(Shutdown::Both);
    Ok((elapsed, total_bytes))
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("usage: tcp_benchmark <addr> <payload_size> <iterations> [concurrency]");
        std::process::exit(1);
    }

    let addr = &args[1];
    let payload_size: usize = args[2].parse()?;
    let iterations: usize = args[3].parse()?;
    let concurrency: usize = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(1);

    // Start a server if none is provided, and use the concrete local address
    // (including the OS-assigned ephemeral port) for clients.
    let (client_addr, _handle) = if addr.starts_with("127.") || addr.starts_with("localhost") {
        let server = ConcurrentServer::bind(addr, 4)?;
        let local = server.local_addr()?;
        println!("started server on {}", local);
        (local.to_string(), Some(server.run(echo_handler)))
    } else {
        (addr.clone(), None)
    };

    thread::sleep(Duration::from_millis(100));

    let mut handles = Vec::with_capacity(concurrency);
    let start = Instant::now();
    for _ in 0..concurrency {
        let client_addr = client_addr.clone();
        handles.push(thread::spawn(move || {
            run_client(&client_addr, payload_size, iterations / concurrency.max(1))
        }));
    }

    let mut total_bytes = 0u64;
    for handle in handles {
        let (_, bytes) = handle.join().unwrap()?;
        total_bytes += bytes;
    }
    let elapsed = start.elapsed();

    let seconds = elapsed.as_secs_f64();
    let requests_per_sec = (iterations as f64) / seconds;
    let bytes_per_sec = (total_bytes as f64) / seconds;
    let latency_ns = (elapsed.as_nanos() as f64) / (iterations as f64);

    println!("payload_size:     {} bytes", payload_size);
    println!("iterations:       {}", iterations);
    println!("concurrency:      {}", concurrency);
    println!("elapsed:          {:.6} s", seconds);
    println!("requests/sec:     {:.2}", requests_per_sec);
    println!("throughput:       {:.2} MB/s", bytes_per_sec / 1_048_576.0);
    println!("avg latency:      {:.0} ns", latency_ns);

    if let Some(handle) = _handle {
        handle.shutdown();
    }

    Ok(())
}
