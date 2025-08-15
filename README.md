# wasm_towertech

A minimal Bevy 0.16 project targeting WebAssembly (WASM) and built with Trunk.

## Features
- Bevy 0.16 game engine
- WASM export using `wasm-bindgen`
- Trunk for asset pipeline and local dev server
- Example: Rotating colored square

## Getting Started

### Prerequisites
- Rust (with `wasm32-unknown-unknown` target)
- [wasm-bindgen-cli](https://github.com/rustwasm/wasm-bindgen)
- [Trunk](https://trunkrs.dev/)

### Build and Run
1. Install Rust WASM target:
   ```sh
   rustup target add wasm32-unknown-unknown
   ```
2. Install wasm-bindgen-cli:
   ```sh
   cargo install wasm-bindgen-cli
   ```
3. Install Trunk:
   ```sh
   cargo install trunk
   ```
4. Build and serve:
   ```sh
   trunk serve
   ```
   Visit [http://127.0.0.1:8080/](http://127.0.0.1:8080/) in your browser.

## Project Structure
- `src/lib.rs`: Bevy app entry point for WASM
- `index.html`: Web entry point (imports generated JS/WASM)
- `Cargo.toml`: Rust dependencies and crate config
- `Trunk.toml`: Trunk configuration

## Notes
- Trunk generates JS/WASM files with a hash in the filename for cache busting.
- Use the import path in `index.html` that matches the generated JS file, or let Trunk rewrite it automatically.
- For more info, see the [Bevy WASM docs](https://bevyengine.org/learn/book/getting-started/setup/#webassembly-wasm).
