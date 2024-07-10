import * as wasm from "./pkg/wgraphcal.js";
import init from "./pkg/wgraphcal.js";

let wasmInitialized = false;

async function wasmInit() {
	await init();
	wasmInitialized = true;
}

export async function ensureWasmInit() {
	if (!wasmInitialized) {
		await wasmInit();
	}
}

export async function scanner() {
	await ensureWasmInit();

	let res = wasm.return_string();
	console.log("promise", res);
	return res;
}
