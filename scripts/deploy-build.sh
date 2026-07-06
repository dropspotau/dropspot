#!/usr/bin/env bash

set -euo pipefail

# Replace the Render URL with a proper one
export DROPSPOT_ENDPOINT="https://${DROPSPOT_ENDPOINT}"
export DATABASE_URL="$DROPSPOT_DATABASE_URL"

cargo install sqlx-cli wasm-pack
./scripts/migrate-database.sh

cargo build --release
./scripts/build-wasm.sh
