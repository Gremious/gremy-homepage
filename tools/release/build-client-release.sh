#!/bin/bash
wasm-pack build client --no-typescript --release --target no-modules --out-dir ../public/wasm
