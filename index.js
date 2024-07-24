import init from "./pkg/wgraphcal.js";
import { faster_call,return_string,get_buf_as_ptr,get_resolution,str_to_lexemes,str_to_tokens,str_to_abstract} from "./pkg/wgraphcal.js";

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

export async function parse_text(input){
  await ensureWasmInit();
  return "Lexemes: "+str_to_lexemes(input)+"\nTokens: "+str_to_tokens(input)+"\n AST: "+str_to_abstract(input);
}


export async function squaredvals(bounds,cw,ch,haxis,vaxis,ssc){
  await ensureWasmInit();
  faster_call(...bounds,cw,ch,haxis,vaxis,ssc,0);
  let a = new Float64Array(rustWasm.memory.buffer).slice(bufferPointer,bufferPointer+(2*(Number(get_resolution())**2)))
//console.log(a);
  return(a);
}

//squaredvals([-10,10,-10,10]);

