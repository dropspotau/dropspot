#!/usr/bin/env bash

set -euo pipefail

if [[ -z $DROPSPOT_DATABASE_URL ]]; then
    echo "DROPSPOT_DATABASE_URL environment variable must be set to a Postgres server's address"
    exit 1
fi

cargo sqlx prepare --workspace
sqlx migrate run --database-url "$DROPSPOT_DATABASE_URL"
