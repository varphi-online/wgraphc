import * as wgraph from "./index.js";
import * as graph from "./graph.js"
let output0;

document.addEventListener("DOMContentLoaded", () => {
  output0 = document.getElementById("out0");
});

async function inpUpdate(input) {
  await wgraph.ensureWasmInit();
  await wgraph.scanner(input)
  if (output0){
  output0.innerText = await wgraph.parse_text(input);
  }

  graph.render();
}

window.inpUpdate = inpUpdate;
