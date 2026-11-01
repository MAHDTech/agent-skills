#!/usr/bin/env bash
set -euo pipefail

if [[ $# -gt 0 ]]; then
	exec cargo nextest run --workspace --all-targets --locked "$@"
fi

cargo nextest run --workspace --all-targets --locked
cargo test --workspace --doc --locked
