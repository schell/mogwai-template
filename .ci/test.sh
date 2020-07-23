#!/bin/sh -eu

set +eu
if [ -f ~/.profile ]; then
    source ~/.profile
fi
set -eu

ROOT="$(git rev-parse --show-toplevel)"
source $ROOT/.ci/rust/common.sh

cargo generate --git git@github.com:schell/mogwai-template.git --name gen-test
cd gen-test
cargo build
