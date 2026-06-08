#!/usr/bin/env bash

set -euo pipefail

tsc
vite build

if [[ -d ../static/dist ]]; then
    # Clear out any previous assets
    rm -rf ../static/dist
fi

mv dist ../static/
