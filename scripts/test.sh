#!/bin/sh -eu

ROOT="$(git rev-parse --show-toplevel)"
. $ROOT/scripts/common.sh

BRANCH="$(git rev-parse --abbrev-ref HEAD)"
echo "Testing project generation from the $BRANCH branch..."
cargo generate --git git@github.com:schell/mogwai-template.git --name gen-test --branch $BRANCH
cd gen-test
cargo build
