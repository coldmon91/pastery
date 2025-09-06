# !/bin/bash

# 1. Start the Rust backend
cargo run --bin pastery

# 2. Start the Tauri frontend
npm run tauri dev