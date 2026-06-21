// Run a wasm32-wasip1 module with Node's built-in WASI.
// Usage: node tests/run_wasi.mjs <module.wasm>
import { WASI } from 'node:wasi';
import { readFile } from 'node:fs/promises';

const wasi = new WASI({ version: 'preview1', args: ['wasm_smoke'], env: {} });
const bytes = await readFile(process.argv[2]);
const module = await WebAssembly.compile(bytes);
const instance = await WebAssembly.instantiate(module, wasi.getImportObject());
process.exit(wasi.start(instance) ?? 0);
