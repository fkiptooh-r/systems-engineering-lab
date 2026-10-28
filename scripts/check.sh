#!/usr/bin/env bash
# Run formatting, lint, and test checks for every Cargo workspace in the lab.
# Usage: scripts/check.sh            # all workspaces
#        scripts/check.sh 01-rust-fundamentals
set -euo pipefail

cd "$(dirname "$0")/.."

if [[ $# -gt 0 ]]; then
    workspaces=("$@")
else
    mapfile -t workspaces < <(find . -mindepth 2 -maxdepth 2 -name Cargo.toml -printf '%h\n' | sort)
fi

for ws in "${workspaces[@]}"; do
    echo "==> $ws"
    (
        cd "$ws"
        cargo fmt --all -- --check
        cargo clippy --all-targets -- -D warnings
        cargo test --quiet
    )
done

echo "All checks passed."
