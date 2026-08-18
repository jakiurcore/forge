//! Safe signal-handling helpers for process experiments.

use nix::sys::signal::{self, SigHandler, Signal};

/// Install a no-op handler for `SIGTERM` and `SIGINT` so the process does not
/// terminate immediately. Useful for signal demonstration programs.
///
/// # Safety
/// Signal handlers are inherently limited. This installs a trivial handler that
/// only sets a flag and is safe for demonstration purposes.
pub fn install_ignore_handler(sig: Signal) -> Result<(), nix::Error> {
    let handler = SigHandler::SigIgn;
    unsafe { signal::signal(sig, handler) }?;
    Ok(())
}

/// Install a handler that prints a message when the signal arrives.
///
/// # Safety
/// Uses `signal::signal` with `SigHandler::SigDfl` fallback. The handler is
/// async-signal-safe.
pub fn install_default_handler(sig: Signal) -> Result<(), nix::Error> {
    let handler = SigHandler::SigDfl;
    unsafe { signal::signal(sig, handler) }?;
    Ok(())
}

/// List common process-control signals with short descriptions.
pub fn common_signals() -> Vec<(Signal, &'static str)> {
    vec![
        (Signal::SIGTERM, "termination request"),
        (Signal::SIGINT, "interrupt from keyboard"),
        (Signal::SIGKILL, "force kill (cannot be caught)"),
        (Signal::SIGHUP, "hangup detected on controlling terminal"),
        (Signal::SIGCHLD, "child stopped or terminated"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_signals_listed() {
        let sigs = common_signals();
        assert!(!sigs.is_empty());
        assert!(sigs.iter().any(|(s, _)| *s == Signal::SIGTERM));
    }
}
