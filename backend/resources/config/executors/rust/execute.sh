#!/bin/bash
set -e

SRC_FILE="$1"
OUT_DIR="$(dirname "$SRC_FILE")"

if [[ -z "$SRC_FILE" || -z "$OUT_DIR" ]]; then
    # echo "Usage: $0 <source.rs>"
    exit 1
fi

BASENAME=$(basename "$SRC_FILE")
BASENAME="${BASENAME%.*}"
BIN_PATH="$OUT_DIR/$BASENAME"

RUST_FILE="$OUT_DIR/$BASENAME.rs"

rustc "$RUST_FILE" -o "$BIN_PATH" > /dev/null

cd "$OUT_DIR"
"./$BASENAME"
