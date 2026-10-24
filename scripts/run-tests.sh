#!/usr/bin/env bash
set -euo pipefail
exec cargo nextest run --workspace --all-targets "$@"
