#!/usr/bin/env bash
set -e
cd "$(dirname "$0")"

cd ../packages/native 
yarn
yarn build