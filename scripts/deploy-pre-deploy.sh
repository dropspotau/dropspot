#!/usr/bin/env bash

set -euo pipefail

cargo install sqlx
./scripts/migrate-database.sh
