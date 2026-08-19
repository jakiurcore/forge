//! UDP request/response throughput benchmark.
//!
//! Run:
//!
//! ```bash
//! cargo run --release --example udp_benchmark -- 127.0.0.1:0 1024 10000
//! ```

use std::env;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("usage: udp_benchmark <addr> <payload_size> <iterations>");
        std::process::exit(1);
    }

    let addr = &args[1];
    let payload_size: usize = args[2].parse()?;
    let iterations: usize = args[3].parse()?;

    let payload = vec![0xCDu8; payload_size];

    let (server_socket, server_local) = forge_network::udp::bind_udp(addr)?;
    println!("udp server listening on {}", server_local);

    let client = std::thread::spawn(move || {
        let (socket, _) = forge_network::udp::bind_udp("127.0.0.1:0")?;
        socket.set_read_timeout(Some(Duration::from_secs(5)))?;

        let start = Instant::now();
        let mut received = 0usize;
        for _ in 0..iterations {
            socket.send_to(&payload, server_local)?;
            let mut buf = vec![0u8; payload_size];
            let (n, _) = socket.recv_from(&mut buf)?;
            received += n;
        }
        let elapsed = start.elapsed();
        Ok::<_, Box<dyn std::error::Error + Send + Sync>>((elapsed, received))
    });

    // Server echoes each datagram back.
    let mut total_echoed = 0usize;
    let mut buf = vec![0u8; payload_size];
    while total_echoed < iterations * payload_size {
        let (n, peer) = server_socket.recv_from(&mut buf)?;
        server_socket.send_to(&buf[..n], peer)?;
        total_echoed += n;
    }

    let (elapsed, received) = client.join().unwrap()?;
    let seconds = elapsed.as_secs_f64();
    let datagrams_per_sec = iterations as f64 / seconds;
    let bytes_per_sec = (received as f64 * 2.0) / seconds; // request + response

    println!("payload_size:     {} bytes", payload_size);
    println!("iterations:       {}", iterations);
    println!("elapsed:          {:.6} s", seconds);
    println!("datagrams/sec:    {:.2}", datagrams_per_sec);
    println!("throughput:       {:.2} MB/s", bytes_per_sec / 1_048_576.0);

    Ok(())
}
