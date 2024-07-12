import * as wgraph from "./index.js";
let output0;

document.addEventListener("DOMContentLoaded", () => {
  output0 = document.getElementById("out0");
});

async function inpUpdate(input) {
  await wgraph.ensureWasmInit();
  output0.innerHTML = "Lexemes: " + (await wgraph.scanner(input));
}

window.inpUpdate = inpUpdate;
