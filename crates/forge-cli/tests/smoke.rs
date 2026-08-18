//! Smoke tests for the Forge CLI.

use std::path::PathBuf;
use std::process::Command;

#[test]
fn forge_status_runs() {
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = crate_dir
        .parent()
        .expect("crate dir has no parent")
        .parent()
        .expect("crate dir has no grandparent");

    let output = Command::new("cargo")
        .args(["run", "--bin", "forge", "--", "status"])
        .current_dir(workspace_root)
        .output()
        .expect("failed to execute forge status");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "forge status exited with failure:\n{}",
        stdout
    );
    assert!(stdout.contains("Forge"));
    assert!(stdout.contains("Day:"));
    assert!(stdout.contains("Phase:"));
}
