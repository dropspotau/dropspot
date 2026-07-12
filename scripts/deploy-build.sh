#!/usr/bin/env bash

set -euo pipefail

if [[ $# -eq 0 ]]; then
    echo "First argument must state \"migrate\" or \"no-migrate\""
    exit 1
fi

if [[ $1 == "migrate" ]]; then
    SHOULD_MIGRATE=true
elif [[ $1 == "no-migrate" ]]; then
    SHOULD_MIGRATE=false
else 
    echo "First argument must state \"migrate\" or \"no-migrate\""
    exit 1
fi

# Replace the Render URL with a proper one
export DROPSPOT_ENDPOINT="https://${DROPSPOT_ENDPOINT}"
export DATABASE_URL="$DROPSPOT_DATABASE_URL"

cargo install wasm-pack

if [[ $SHOULD_MIGRATE == true ]]; then
    echo "Migrating database"
    echo "Database URL: $DATABASE_URL"
    cargo install sqlx-cli
    ./scripts/migrate-database.sh
fi

cargo build --release
./scripts/build-wasm.sh
