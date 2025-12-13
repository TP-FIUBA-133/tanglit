#!/bin/bash
set -e

SRC_FILE="$1"
OUT_DIR="$(dirname "$SRC_FILE")"

if [[ -z "$SRC_FILE" || -z "$OUT_DIR" ]]; then
    # echo "Usage: $0 <source.c>"
    exit 1
fi

BASENAME=$(basename "$SRC_FILE")
BASENAME="${BASENAME%.*}"
BIN_PATH="$OUT_DIR/$BASENAME"

C_FILE="$OUT_DIR/$BASENAME.c"

gcc -std=c99 -pedantic-errors -o "$BIN_PATH" "$C_FILE" > /dev/null

cd "$OUT_DIR"
"./$BASENAME"
