#!/bin/bash
set -e

SRC_FILE="$1"
OUT_DIR="$(dirname "$SRC_FILE")"

if [[ -z "$SRC_FILE" || -z "$OUT_DIR" ]]; then
    # echo "Usage: $0 <source.hs> <output_dir>"
    exit 1
fi

BASENAME=$(basename "$SRC_FILE")
BASENAME="${BASENAME%.*}"
BIN_PATH="$OUT_DIR/$BASENAME"

CPP_FILE="$OUT_DIR/$BASENAME.cpp"
mv "$SRC_FILE" "$CPP_FILE"

g++ -o "$BIN_PATH" "$CPP_FILE" > /dev/null

cd "$OUT_DIR"
"./$BASENAME"