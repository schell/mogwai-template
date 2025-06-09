#!/bin/sh -eu

export PATH=$HOME/.cargo/bin:$PATH:

echo "Rust Setup"

if hash rustup 2>/dev/null; then
    echo "Have rustup, skipping installation..."
else
    echo "Installing rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi

rustup update

if hash trunk 2>/dev/null; then
    echo "Have trunk, skipping installation..."
else
    echo "Installing trunk..."
    cargo install --locked trunk
fi

if hash cargo-generate 2>/dev/null; then
    echo "Have cargo-generate, skipping installation..."
else
    echo "Installing cargo-generate..."
    cargo install cargo-generate
    which cargo-generate
fi
