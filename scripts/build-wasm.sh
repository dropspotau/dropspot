#!/usr/bin/env bash

set -euo pipefail

base_dir=$(pwd)

if [[ ! -d "core" || ! -d "web" ]]; then
    echo "Could not find core and web directories. This should be run from the DropSpot project root directory."
    exit 1
fi

# Assume this is run in the project root
cd "$base_dir/core" || exit 1
wasm-pack build --target web --scope dropspot

cd "$base_dir/web" || exit 1
rm -rf node_modules
pnpm install
pnpm build

echo "Built WASM package and installed to frontend!"
