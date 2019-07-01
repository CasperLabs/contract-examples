#!/usr/bin/env bash

set -o errexit

ARCH="wasm32-unknown-unknown"

# This is necessary for CI
source "${HOME}/.cargo/env"

# This is also necessary for CI
rustup toolchain install $(cat rust-toolchain)

# This is also necessary for CI
rustup target add --toolchain $(cat rust-toolchain) $ARCH

cargo build --release
