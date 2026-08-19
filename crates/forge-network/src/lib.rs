//! Networking primitives and experiments for Forge.
//!
//! This crate provides synchronous TCP/UDP helpers, length-prefixed framing,
//! a small Forge application protocol, and a concurrent TCP server built on
//! top of `forge-concurrency::ThreadPool`.

#![deny(missing_docs)]

pub mod address;
pub mod error;
pub mod framing;
pub mod protocol;
pub mod server;
pub mod tcp;
pub mod timeout;
pub mod udp;

pub use address::{bind_listener, resolve};
pub use framing::{FrameCodec, HEADER_LEN, MAX_FRAME_SIZE};
pub use protocol::{Command, Request, Response, Status, MAX_PAYLOAD_SIZE};
pub use server::{ConcurrentServer, ServerHandle};
pub use tcp::{bind_and_listen, connect, echo_handler, run_echo_server, send_message};
pub use timeout::{connect_with_timeout, set_stream_timeouts};
