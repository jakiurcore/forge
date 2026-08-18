#!/usr/bin/env bash
# Forge validation script.
# Runs the full Rust validation chain expected by CI.

set -euo pipefail

cd "$(dirname "$0")/.."

echo "==> Running cargo fmt --check"
cargo fmt --check

echo "==> Running cargo clippy --all-targets --all-features"
cargo clippy --all-targets --all-features -- -D warnings

echo "==> Running cargo test --workspace"
cargo test --workspace

echo "==> Running cargo build --workspace"
cargo build --workspace

echo "==> Validation complete"
