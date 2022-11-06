#!/bin/bash
set -eu

rustup target add wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli

cargo install --locked trunk