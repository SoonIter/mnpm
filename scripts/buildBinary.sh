#!/usr/bin/env bash

export RUSTFLAGS="-C strip=symbols"

target="$TARGET"

echo "Target: $target"
echo "Args: $@"

# Build the binary with the provided target
# rustup target add "$target"
cargo build --release --target "$target" $@
