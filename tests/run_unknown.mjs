// Run a wasm32-unknown-unknown module (no WASI). The smoke example exports
// `main`; instantiate and call it. Return value 0 means all asserts passed
// (there is no stdout on this target).
// Usage: node tests/run_unknown.mjs <module.wasm>
//
// set_by_csprng (if used) imports `env.mclRustFillRandom(ptr, len)` from the
// host; back it with Web Crypto. The smoke example does not call it, so the
// import is dead-code-eliminated, but wire it up so modules that do work too.
import { readFile } from 'node:fs/promises';
import { webcrypto } from 'node:crypto';

const bytes = await readFile(process.argv[2]);
const module = await WebAssembly.compile(bytes);

let instance;
const imports = {
  env: {
    mclRustFillRandom(ptr, len) {
      const mem = new Uint8Array(instance.exports.memory.buffer, ptr, len);
      webcrypto.getRandomValues(mem);
    },
  },
};
instance = await WebAssembly.instantiate(module, imports);
const entry = instance.exports.main ?? instance.exports.__main_void;
const r = entry(0, 0);
console.log('main returned', r);
process.exit(r === 0 ? 0 : 1);
