#!/usr/bin/env bash

set -euo pipefail

base_dir=$(pwd)

if [[ ! -d "core" || ! -d "static/dropspot" ]]; then
    echo "Could not find core and static directories. This should be run from the root directory."
    exit 1
fi

# Assume this is run in the project root
cd "$base_dir/core" || exit 1
wasm-pack build --target web

cd "$base_dir/static/dropspot" || exit 1
ls

rm -rf node_modules
pnpm install

echo "Built WASM package and installed to frontend!"
