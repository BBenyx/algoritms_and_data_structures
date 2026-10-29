#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")/.."

if [[ -z "${1:-}" ]]; then
    echo "Error! No file name parameter passed." >&2
    exit 1
fi

filename="$1"
echo "$filename"

# Running rust sorting_bench
cargo bench --bench sorting_bench -- "$filename"

# Running R analysis
Rscript analysis/report.r "data/${filename}.csv"