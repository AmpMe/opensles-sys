#!/usr/bin/env bash
set -e
CUR_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd $CUR_DIR

GENERATE_BINDINGS=1 cargo build --target armv7-linux-androideabi
