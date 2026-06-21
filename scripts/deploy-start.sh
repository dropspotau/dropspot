#!/usr/bin/env bash

set -euo pipefail

cargo run --release --package dropspot-server server run
