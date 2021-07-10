watchexec -r -s SIGKILL -w "client" -w "shared" -w "server" -w "hobo" "wasm-pack build client --dev --target web --out-dir ../public/wasm && cargo run --package server"
