cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --target web `
--out-dir dist `
target\wasm32-unknown-unknown\release\wasm_towertech.wasm
cp .\index.html .\dist\