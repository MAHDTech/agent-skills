#!/usr/bin/env bash
set -euo pipefail

# Total Recall decision logging helper
# Usage: log.sh <logfile> <phase> <decision> <why> <evidence> <result>

if [ "$#" -lt 6 ]; then
  echo "Usage: $0 <logfile> <phase> <decision> <why> <evidence> <result>" >&2
  exit 1
fi

LOGFILE="$1"
PHASE="$2"
DECISION="$3"
WHY="$4"
EVIDENCE="$5"
RESULT="$6"

# Ensure parent directory exists
mkdir -p "$(dirname "$LOGFILE")"

# Initialize header on first use if file does not exist or is empty
if [ ! -s "$LOGFILE" ]; then
  printf "ts\tphase\tdecision\twhy\tevidence\tresult\n" > "$LOGFILE"
fi

# Sanitize function: strip tabs, newlines, and prevent formula injection in spreadsheets
sanitize_field() {
  local val="$1"
  # Replace tabs and newlines with spaces
  val=$(printf "%s" "$val" | tr '\t\r\n' '   ')
  # Prefix formula characters (=, +, -, @) with a single quote
  case "$val" in
    [=+@-]* ) val="'$val" ;;
  esac
  printf "%s" "$val"
}

TIMESTAMP=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
S_PHASE=$(sanitize_field "$PHASE")
S_DECISION=$(sanitize_field "$DECISION")
S_WHY=$(sanitize_field "$WHY")
S_EVIDENCE=$(sanitize_field "$EVIDENCE")
S_RESULT=$(sanitize_field "$RESULT")

printf "%s\t%s\t%s\t%s\t%s\t%s\n" "$TIMESTAMP" "$S_PHASE" "$S_DECISION" "$S_WHY" "$S_EVIDENCE" "$S_RESULT" >> "$LOGFILE"
