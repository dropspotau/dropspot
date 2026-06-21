#!/usr/bin/env bash

set -euo pipefail

# Replace the Render URL with a proper one
export DROPSPOT_ENDPOINT="https://${DROPSPOT_ENDPOINT}"

cargo build --release
./scripts/build-wasm.sh
