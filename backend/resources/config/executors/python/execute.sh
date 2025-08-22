#!/bin/bash
set -e

SRC_FILE="$1"
OUT_DIR="$(dirname "$SRC_FILE")"

if [[ -z "$SRC_FILE" || -z "$OUT_DIR" ]]; then
    # echo "Usage: $0 <source.py>"
    exit 1
fi

BASENAME=$(basename "$SRC_FILE")
BASENAME="${BASENAME%.*}"
BIN_PATH="$OUT_DIR/$BASENAME"

PYTHON_FILE="$OUT_DIR/$BASENAME.py"

cd "$OUT_DIR"
python3 "$PYTHON_FILE"
