import init from "./pkg/wgraphcal.js";
import { faster_call,return_string,get_buf_as_ptr,get_resolution} from "./pkg/wgraphcal.js";
export let bufferPointer;
let wasmInitialized = false;
let rustWasm;
var MEM_BUFF;
const runWasm = async () => {
  rustWasm = await init("./pkg/wgraphcal_bg.wasm");
  MEM_BUFF = new Float64Array(rustWasm.memory.buffer)
  bufferPointer = rustWasm.get_buf_as_ptr()/8;
  wasmInitialized = true;
  
}

//async function wasmInit() {
//  await init();
//  wasmInitialized = true;
//}
//
export async function ensureWasmInit() {
  if (!wasmInitialized) {
    await runWasm();
  }
  
}

export async function scanner(input) {
  await ensureWasmInit();

  let res = return_string(input);
  return res;
}


export async function squaredvals(bounds,cw,ch,toggle){
  await ensureWasmInit();
  faster_call(...bounds,cw,ch,toggle,"r","r");
  let a = new Float64Array(rustWasm.memory.buffer).slice(bufferPointer,bufferPointer+(4*(Number(get_resolution())**2)))
//console.log(a);
  return(a);
}

//squaredvals([-10,10,-10,10]);

