#!/usr/bin/env bash

set -euo pipefail

cargo build --release
./scripts/build-wasm.sh
