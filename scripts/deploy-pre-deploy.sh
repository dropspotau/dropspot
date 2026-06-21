#!/usr/bin/env bash

set -euo pipefail

cargo install sqlx
cargo sqlx prepare
./scripts/migrate-database.sh
