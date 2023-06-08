#!/bin/bash
wasm-pack build client --no-typescript --release --target no-modules --out-dir ../public/wasm
sudo systemctl stop homepage
cargo build -p server --release
sudo systemctl start homepage
