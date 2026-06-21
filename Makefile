# Build and run the wasm smoke test (examples/wasm_smoke.rs) on both wasm
# targets using Node only (no wasmtime/wasmer needed).
#
#   wasm32-wasip1            -> WASI (browser/Node share wasm32-unknown-unknown)
#   wasm32-unknown-unknown   -> browser and Node.js (plain WebAssembly API)

WASI_WASM    = target/wasm32-wasip1/debug/examples/wasm_smoke.wasm
UNKNOWN_WASM = target/wasm32-unknown-unknown/debug/examples/wasm_smoke.wasm

.PHONY: wasm-test wasm-targets

wasm-targets:
	rustup target add wasm32-unknown-unknown wasm32-wasip1

wasm-test: wasm-targets
	cargo build --example wasm_smoke --target wasm32-wasip1
	cargo build --example wasm_smoke --target wasm32-unknown-unknown
	@echo "[wasip1]"
	node tests/run_wasi.mjs $(WASI_WASM)
	@echo "[unknown-unknown]"
	node tests/run_unknown.mjs $(UNKNOWN_WASM)
