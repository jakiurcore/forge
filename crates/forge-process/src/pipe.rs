//! Unix pipe helpers for parent/child communication experiments.

use nix::unistd::{fork, pipe, ForkResult};
use std::ffi::CString;
use std::os::fd::IntoRawFd;

/// Create a pipe and fork. The child writes a message to the parent.
///
/// Returns the bytes received by the parent.
pub fn demo_pipe_message(message: &[u8]) -> Result<Vec<u8>, nix::Error> {
    let (read_fd, write_fd) = pipe()?;
    let read_raw = read_fd.into_raw_fd();
    let write_raw = write_fd.into_raw_fd();

    match unsafe { fork() }? {
        ForkResult::Parent { child: _ } => {
            unsafe { libc::close(write_raw) };
            let mut buffer = vec![0u8; message.len()];
            let n = unsafe { libc::read(read_raw, buffer.as_mut_ptr().cast(), buffer.len()) };
            if n < 0 {
                return Err(nix::Error::last());
            }
            buffer.truncate(n as usize);
            unsafe { libc::close(read_raw) };
            Ok(buffer)
        }
        ForkResult::Child => {
            unsafe { libc::close(read_raw) };
            let _ = unsafe { libc::write(write_raw, message.as_ptr().cast(), message.len()) };
            unsafe { libc::close(write_raw) };
            std::process::exit(0);
        }
    }
}

/// Spawn a child process that replaces stdout with the write end of a pipe and
/// executes `program` with `args`. The parent receives the child's stdout.
///
/// This is a low-level demonstration. Prefer `std::process::Command` for
/// production use.
pub fn pipe_exec(program: &str, args: &[&str]) -> Result<Vec<u8>, nix::Error> {
    let (read_fd, write_fd) = pipe()?;
    let read_raw = read_fd.into_raw_fd();
    let write_raw = write_fd.into_raw_fd();

    let program_c = CString::new(program).expect("program contains NUL");
    let arg_c: Vec<CString> = args
        .iter()
        .map(|a| CString::new(*a).expect("arg contains NUL"))
        .collect();

    match unsafe { fork() }? {
        ForkResult::Parent { child: _ } => {
            unsafe { libc::close(write_raw) };
            let mut buffer = Vec::new();
            let mut chunk = [0u8; 1024];
            loop {
                let n = unsafe { libc::read(read_raw, chunk.as_mut_ptr().cast(), chunk.len()) };
                if n <= 0 {
                    break;
                }
                buffer.extend_from_slice(&chunk[..n as usize]);
            }
            unsafe { libc::close(read_raw) };
            Ok(buffer)
        }
        ForkResult::Child => {
            unsafe { libc::close(read_raw) };
            if unsafe { libc::dup2(write_raw, libc::STDOUT_FILENO) } < 0 {
                std::process::exit(127);
            }
            unsafe { libc::close(write_raw) };
            let mut argv: Vec<&CString> = arg_c.iter().collect();
            argv.insert(0, &program_c);
            let _ = nix::unistd::execvp(&program_c, &argv);
            std::process::exit(127);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_pipe_roundtrip() {
        let message = b"hello from child";
        let received = demo_pipe_message(message).unwrap();
        assert_eq!(&received, message);
    }

    #[test]
    fn pipe_exec_runs_echo() {
        let output = pipe_exec("/bin/echo", &["forge-pipe-test"]).unwrap();
        let text = String::from_utf8_lossy(&output);
        assert!(text.contains("forge-pipe-test"));
    }
}
