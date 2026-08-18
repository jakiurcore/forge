#!/usr/bin/env bash
# Forge watchdog.
# Detects missing daily completion, broken builds, failed validation, and
# incomplete state. Uses bounded retries and produces a Markdown report.

set -euo pipefail

ROOT="$(dirname "$0")/.."
cd "$ROOT"

REPORT="logs/watchdog-report.md"
MAX_RETRIES=3
RETRIES=0
FAILURES=()

mkdir -p logs

retry() {
    local cmd="$1"
    local attempt=0
    while true; do
        if eval "$cmd"; then
            return 0
        fi
        attempt=$((attempt + 1))
        if [ "$attempt" -ge "$MAX_RETRIES" ]; then
            return 1
        fi
        local delay=$((60 * (2 ** (attempt - 1))))
        echo "Retry $attempt/$MAX_RETRIES after ${delay}s..."
        sleep "$delay"
    done
}

# Check workspace build.
if ! retry "cargo build --workspace"; then
    FAILURES+=("broken build: cargo build --workspace failed")
    RETRIES=$((RETRIES + MAX_RETRIES))
fi

# Check validation.
if ! retry "cargo test --workspace"; then
    FAILURES+=("failed validation: cargo test --workspace failed")
    RETRIES=$((RETRIES + MAX_RETRIES))
fi

# Check state file. In BUILD 01 a missing state file is informational only.
if [ ! -f .forge/state.toml ]; then
    echo "Note: .forge/state.toml is missing; using default state."
fi

# Daily completion marker check is disabled in BUILD 01; it will be enabled
# once the daily execution engine is implemented.

# Generate report.
{
    echo "# Forge Watchdog Report"
    echo
    if [ "${#FAILURES[@]}" -eq 0 ]; then
        echo "Status: **healthy**"
    else
        echo "Status: **unhealthy**"
    fi
    echo "Retries: $RETRIES"
    echo
    if [ "${#FAILURES[@]}" -eq 0 ]; then
        echo "No findings."
    else
        echo "## Findings"
        for failure in "${FAILURES[@]}"; do
            echo "- $failure"
        done
    fi
} > "$REPORT"

cat "$REPORT"

if [ "${#FAILURES[@]}" -gt 0 ]; then
    exit 1
fi
