#!/usr/bin/env bash

set -euo pipefail

if [[ $# -gt 0 && $1 == "migrate" ]]; then
    SHOULD_MIGRATE=$1
else
    SHOULD_MIGRATE=false
fi

# Replace the Render URL with a proper one
export DROPSPOT_ENDPOINT="https://${DROPSPOT_ENDPOINT}"
export DATABASE_URL="$DROPSPOT_DATABASE_URL"

cargo install sqlx-cli wasm-pack

if [[ $SHOULD_MIGRATE == "true" ]]; then
    echo "Migrating database"
    ./scripts/migrate-database.sh
fi

cargo build --release
./scripts/build-wasm.sh
