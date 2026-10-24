#!/usr/bin/env sh
set -eu

if [ $# -eq 0 ]; then
    echo "Usage: bin/demo.sh <token-or-phrase>" >&2
    exit 2
fi

cd "$(dirname "$0")/.."

cargo build
./target/debug/translit convert "$1" --schema mosmetro
