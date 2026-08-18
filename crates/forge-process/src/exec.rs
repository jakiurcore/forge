//! `fork()` + `exec()` demonstration helpers.

use crate::error::ProcessError;
use nix::unistd::{execvp, fork, ForkResult};
use std::ffi::CString;

/// Fork and exec a program in the child, returning the child's PID to the parent.
///
/// This is a low-level demonstration. Production code should use
/// `std::process::Command`.
pub fn fork_exec(program: &str, args: &[&str]) -> Result<i32, ProcessError> {
    let program_c = CString::new(program).map_err(|e| ProcessError::Other(e.to_string()))?;
    let mut arg_c: Vec<CString> = Vec::new();
    for arg in args {
        arg_c.push(CString::new(*arg).map_err(|e| ProcessError::Other(e.to_string()))?);
    }

    match unsafe { fork() }.map_err(|e| ProcessError::SystemError(e.into()))? {
        ForkResult::Parent { child } => Ok(child.as_raw()),
        ForkResult::Child => {
            let mut argv: Vec<&CString> = arg_c.iter().collect();
            argv.insert(0, &program_c);
            let _ = execvp(&program_c, &argv);
            std::process::exit(127);
        }
    }
}
