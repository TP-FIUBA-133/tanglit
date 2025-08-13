#!/bin/bash
set -e

SRC_FILE="$1"
OUT_DIR="$(dirname "$SRC_FILE")"

if [[ -z "$SRC_FILE" || -z "$OUT_DIR" ]]; then
    # echo "Usage: $0 <source.hs>"
    exit 1
fi

BASENAME="${SRC_FILE##*/}"  # Get filename without path
BASENAME="${BASENAME%.*}"   # Remove extension
BIN_PATH="$OUT_DIR/$BASENAME"

# add .hs extension to the basename
HS_FILE="$OUT_DIR/$BASENAME.hs"
mv "$SRC_FILE" "$HS_FILE"

ghc -outputdir "$OUT_DIR" -o "$BIN_PATH" "$HS_FILE" > /dev/null

cd "$OUT_DIR"
"./$BASENAME"
