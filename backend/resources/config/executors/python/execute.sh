#!/bin/bash
set -e

unset PYTHONHOME
unset PYTHONPATH

SRC_FILE="$1"
OUT_DIR="$(dirname "$SRC_FILE")"

if [[ -z "$SRC_FILE" || -z "$OUT_DIR" ]]; then
    # echo "Usage: $0 <source.py>"
    exit 1
fi

BASENAME=$(basename "$SRC_FILE")
PYTHON_FILE="$BASENAME"

cd "$OUT_DIR"
python3 "$PYTHON_FILE"
