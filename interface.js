import * as wgraph from "./index.js";
import * as graph from "./graph.js"
let output0;

document.addEventListener("DOMContentLoaded", () => {
  output0 = document.getElementById("out0");
});

async function inpUpdate(input) {
  await wgraph.ensureWasmInit();
  output0.innerHTML = "Lexemes: " + (await wgraph.scanner(input));
  graph.render();
}

window.inpUpdate = inpUpdate;
