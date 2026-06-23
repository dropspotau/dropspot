#!/usr/bin/env bash

set -euo pipefail

# Replace the Render URL with a proper one
export DROPSPOT_ENDPOINT="https://${DROPSPOT_ENDPOINT}"
export DATABASE_URL="$DROPSPOT_DATABASE_URL"
export PORT="$DROPSPOT_PORT" # Render uses the PORT environment variable

cargo run --release --package dropspot-server server run
