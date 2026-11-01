#!/usr/bin/env bash
# Development only: ask the running `npm run dev:fixtures` instance for a panel
# capture. Writes <prefix>.png and <prefix>.json, prints the JSON, and fails when
# the capture failed (see src-tauri/src/capture.rs).
#
#   scripts/capture-panel.sh /abs/path/prefix [WIDTHxHEIGHT] [dark|light|system]
set -euo pipefail

prefix=${1:?usage: capture-panel.sh /abs/prefix [WIDTHxHEIGHT] [dark|light|system]}
size=${2:-1100x700}
root=$(cd "$(dirname "$0")/.." && pwd)
binary=$root/src-tauri/target/debug/kvojps-monitor

pgrep -f "^$binary\$|^target/debug/kvojps-monitor\$" > /dev/null \
  || { echo "start the app first: npm run dev:fixtures" >&2; exit 1; }

args=(--capture "$prefix" --size "$size")
[ -n "${3:-}" ] && args+=(--theme "$3")
rm -f "$prefix.png" "$prefix.json"
"$binary" "${args[@]}"

for _ in $(seq 1 40); do
  [ -f "$prefix.json" ] && break
  sleep 1
done
[ -f "$prefix.json" ] || { echo "no answer from the running instance" >&2; exit 1; }
cat "$prefix.json"
! grep -q '"error"' "$prefix.json"
