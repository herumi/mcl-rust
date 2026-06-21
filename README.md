# mcl for Rust

This is a wrapper library of [mcl](https://github.com/herumi/mcl/),
which is a portable and fast pairing-based cryptography library.

# Test

```
git clone --recursive https://github.com/herumi/mcl-rust
cd mcl-rust
cargo build
cargo test
```

# WebAssembly

There are three usage environments but only two Rust build targets.
A browser and Node.js share the same `wasm32-unknown-unknown` artifact; only the
surrounding JS glue differs. WASI is the separate one.

| environment | how it loads the wasm | Rust target |
|---|---|---|
| browser | `WebAssembly.instantiate` (+ JS glue) | `wasm32-unknown-unknown` |
| Node.js (non-WASI) | `WebAssembly.instantiate` | `wasm32-unknown-unknown` |
| WASI (wasmtime / `node:wasi` / ...) | WASI runtime (`_start`) | `wasm32-wasip1` |

Building requires `clang++` and `llvm-ar` (the wasm build of mcl compiles
`mcl/src/fp.cpp` directly). Running the test below needs only **Node.js**.

To pick a specific LLVM toolchain, set `CLANG_VER` as a suffix (same convention
as `mcl/Makefile.wasm`), e.g. `CLANG_VER=-18` uses `clang++-18` / `llvm-ar-18`:

```
CLANG_VER=-18 make wasm-test
```

`CXX` / `AR` override the compiler / archiver outright if set.

The deterministic smoke test is `examples/wasm_smoke.rs` (Fr arithmetic, pairing
bilinearity, and `mul_vec`). Run both targets at once with:

```
make wasm-test
```

Or run each target manually:

1. WASI (`wasm32-wasip1`)

```
rustup target add wasm32-wasip1
cargo build --example wasm_smoke --target wasm32-wasip1
node tests/run_wasi.mjs target/wasm32-wasip1/debug/examples/wasm_smoke.wasm
# => ok
```

2. browser / Node.js (`wasm32-unknown-unknown`)

```
rustup target add wasm32-unknown-unknown
cargo build --example wasm_smoke --target wasm32-unknown-unknown
node tests/run_unknown.mjs target/wasm32-unknown-unknown/debug/examples/wasm_smoke.wasm
# => main returned 0
```

`wasm32-unknown-unknown` has no WASI entry and only exports `main`, so it is run
by instantiating with no imports and calling `main` (its return value is checked
instead of stdout). For WASI you may also use a standalone runtime if installed,
e.g. `wasmtime run target/wasm32-wasip1/debug/examples/wasm_smoke.wasm`
(install: `curl https://wasmtime.dev/install.sh -sSf | bash`).

Note: `set_by_csprng` is not available on wasm; supply randomness on the Rust
side (e.g. via `set_little_endian_mod`).

# License

modified new BSD License
http://opensource.org/licenses/BSD-3-Clause

# Author

光成滋生 MITSUNARI Shigeo(herumi@nifty.com)
