#!/bin/sh -eu

section() {
    echo "--- $(TZ=UTC date +%Y%m%d-%H:%M:%S) - $1"
}

section "Rust Setup"

if hash rustup 2>/dev/null; then
    echo "Have rustup, skipping installation..."
else
    echo "Installing rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi

rustup update
cargo install cargo-generate
