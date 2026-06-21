// Run a wasm32-unknown-unknown module (no WASI). The smoke example exports
// `main`; instantiate with no imports and call it. Return value 0 means all
// asserts passed (there is no stdout on this target).
// Usage: node tests/run_unknown.mjs <module.wasm>
import { readFile } from 'node:fs/promises';

const bytes = await readFile(process.argv[2]);
const module = await WebAssembly.compile(bytes);
const instance = await WebAssembly.instantiate(module, {});
const entry = instance.exports.main ?? instance.exports.__main_void;
const r = entry(0, 0);
console.log('main returned', r);
process.exit(r === 0 ? 0 : 1);
