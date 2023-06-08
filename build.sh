#!/bin/bash
wasm-pack build client --no-typescript --dev --target no-modules --out-dir ../public/wasm
sudo systemctl stop homepage
cargo build -p server
sudo systemctl start homepage
