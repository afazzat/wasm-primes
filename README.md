# WASM Prime Generator

A Rust WebAssembly for fun:
- Prime number generation using Sieve of Eratosthenes
- Error handling between Rust and JavaScript
- WASM optimization techniques

## Setup

```bash
# 1. Install Rust and wasm-pack
curl https://sh.rustup.rs -sSf | sh
cargo install wasm-pack

# 2. Build optimized (for production) WASM module
wasm-pack build --target web --release

# 3. Run Rust unit tests
cargo test --lib

# 4. Run the demo
python3 -m http.server 8000
Open http://localhost:8000
