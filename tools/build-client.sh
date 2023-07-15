#!/bin/bash
wasm-pack build client --no-typescript --dev --target no-modules --out-dir ../public/wasm
