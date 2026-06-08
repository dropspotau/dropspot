#!/usr/bin/env bash

set -euo pipefail

tsc
vite build
mv dist ../static/
