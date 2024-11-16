#!/bin/bash

set -euo pipefail

TARGET=wasm32-unknown-unknown
BINARY=pkg/bolt_bg.wasm

# cargo make build

# # NEW PART:
wasm-snip --snip-rust-fmt-code \
          --snip-rust-panicking-code \
          -o $BINARY \
          $BINARY

wasm-strip $BINARY
# wasm-strip dist/rate-lc_bg.wasm
# wasm-gc $BINARY -o $BINARY
wasm-opt --strip-debug -o $BINARY -Oz $BINARY --dce --vacuum
ls -lh $BINARY

# wasm-objdump -x pkg/bolt_bg.wasm > wasm_dump.txt  