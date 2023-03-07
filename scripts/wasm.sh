#!/usr/bin/env bash
set -e
cd "$(dirname "$0")"

cd ../crates/wasm

wasm-pack build --target nodejs --release --out-dir ../../packages/wasm