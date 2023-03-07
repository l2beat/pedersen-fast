#!/usr/bin/env bash
set -e
cd "$(dirname "$0")"

echo "Building packages..."
./native.sh
./wasm.sh

cd ../packages/benchmark
yarn
yarn dev