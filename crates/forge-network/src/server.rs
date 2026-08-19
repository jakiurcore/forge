//! A concurrent TCP server built on top of `forge-concurrency::ThreadPool`.
//!
//! Architecture:
//!
//! ```text
//! TCP Listener
//!      │
//!   accept()
//!      │
//!      ├── Connection → ThreadPool → handler
//!      ├── Connection → ThreadPool → handler
//!      └── Connection → ThreadPool → handler
//! ```
//!
//! The server uses a non-blocking accept loop and a shutdown channel so that
//! tests and examples can stop it cleanly. This is an educational
//! implementation; production servers usually need finer-grained shutdown and
//! backpressure.

use crate::error::NetworkError;
use crate::tcp::bind_and_listen;
use forge_concurrency::ThreadPool;
use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::sync::mpsc::{self, Sender};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

/// A concurrent TCP server.
#[derive(Debug)]
pub struct ConcurrentServer {
    listener: TcpListener,
    pool: ThreadPool,
}

/// Handle to a running server. Dropping it shuts the server down and waits
/// for the accept thread to finish.
pub struct ServerHandle {
    shutdown: Option<Sender<()>>,
    thread: Option<JoinHandle<()>>,
    local_addr: SocketAddr,
}

impl ServerHandle {
    /// Signal the server to shut down and wait for the accept thread.
    pub fn shutdown(mut self) {
        if let Some(tx) = self.shutdown.take() {
            let _ = tx.send(());
        }
        // Connect to the server to unblock the blocking accept call.
        let _ = TcpStream::connect(self.local_addr);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

impl Drop for ServerHandle {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown.take() {
            let _ = tx.send(());
        }
        let _ = TcpStream::connect(self.local_addr);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

impl ConcurrentServer {
    /// Bind a new concurrent server to `addr` with a thread pool of
    /// `pool_size` workers.
    pub fn bind<A: ToSocketAddrs>(addr: A, pool_size: usize) -> Result<Self, NetworkError> {
        let (listener, _) = bind_and_listen(addr)?;
        let pool = ThreadPool::new(pool_size)?;
        Ok(Self { listener, pool })
    }

    /// The local address the server is bound to.
    pub fn local_addr(&self) -> Result<SocketAddr, NetworkError> {
        Ok(self.listener.local_addr()?)
    }

    /// Start the accept loop, dispatching each connection to the thread pool.
    ///
    /// The returned `ServerHandle` keeps the server alive. Call
    /// `handle.shutdown()` to stop accepting new connections and join the
    /// accept thread.
    pub fn run<F>(self, handler: F) -> ServerHandle
    where
        F: Fn(TcpStream) + Send + Sync + 'static,
    {
        let local_addr = self.local_addr().expect("listener has local address");
        let (shutdown_tx, shutdown_rx) = mpsc::channel::<()>();
        let handler = Arc::new(handler);

        let thread = thread::spawn(move || {
            for stream in self.listener.incoming() {
                // Check shutdown after each accept so the handle can stop the
                // loop without waiting for another client.
                if shutdown_rx.try_recv().is_ok() {
                    break;
                }
                match stream {
                    Ok(stream) => {
                        let h = Arc::clone(&handler);
                        self.pool.execute(move || h(stream));
                    }
                    Err(_) => break,
                }
            }
        });

        ServerHandle {
            shutdown: Some(shutdown_tx),
            thread: Some(thread),
            local_addr,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tcp::send_message;
    use std::io::{Read, Write};
    use std::time::Duration;

    #[test]
    fn concurrent_server_echoes_clients() {
        let server = ConcurrentServer::bind("127.0.0.1:0", 4).unwrap();
        let local = server.local_addr().unwrap();

        let handle = server.run(|mut stream| {
            let mut buf = [0u8; 64];
            match stream.read(&mut buf) {
                Ok(n) if n > 0 => {
                    let _ = stream.write_all(&buf[..n]);
                }
                _ => {}
            }
        });

        let clients: Vec<_> = (0..8)
            .map(|i| {
                let msg = format!("client {}", i);
                let response = send_message(
                    local,
                    msg.as_bytes(),
                    Duration::from_secs(2),
                    Duration::from_secs(2),
                )
                .unwrap();
                (msg, response)
            })
            .collect();

        for (msg, response) in clients {
            assert_eq!(response, msg.as_bytes());
        }

        handle.shutdown();
    }
}
