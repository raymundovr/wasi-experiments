import { readFile } from "node:fs/promises";
import { WASI } from "wasi";
import { argv } from "node:process";

const wasi = new WASI({
    args: argv,
    preopens: {
        '.': '.',
        '/tmp': '/tmp'
    }
});

const importObject = { wasi_snapshot_preview1: wasi.wasiImport };

const wasm = await WebAssembly.compile(
    await readFile(new URL("./hello_wasmtime.wasm", import.meta.url)),
);

const instance = await WebAssembly.instantiate(wasm, importObject);

wasi.start(instance);