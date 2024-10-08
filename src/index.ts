import {
  canvasContainer,
  graphContainer,
  proceduralOffscreen,
} from "./graph.js";
import { function_text_inputs } from "./metaUI.js";
import { get_wasm_debug } from "./wasm.js";

export let debug_mode = await get_wasm_debug();

let canvas: canvasContainer = new canvasContainer();
canvas.init();

// Initialize the GL context
export const mainCanvasContext: CanvasRenderingContext2D = <
  CanvasRenderingContext2D
  >canvas.object.getContext("2d");
canvas.resetBitmap(mainCanvasContext);
mainCanvasContext.font = "15px serif";
mainCanvasContext.strokeStyle = "white";
mainCanvasContext.lineWidth = 3;
mainCanvasContext.textAlign = "center";

// Initialize global graph obj
export let graph = new graphContainer(canvas);
graph.init();

// Initialize global variable map
export let var_map = "{}";

export function set_varmap(map: string) {
  var_map = map;
}

// Map of offscreen canvases to be drawn to
let function_array = new Map<number, proceduralOffscreen>();

// Object of inputs to build above map
let inputs = new function_text_inputs(function_array);

canvas.event_init(canvas, graph);

export async function render() {
  await graph.render(
    canvas,
    mainCanvasContext,
    graph,
    Array.from(function_array.values()),
    var_map,
  );
}

window.addEventListener("resize", async function() {
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
  canvas.resetBitmap(mainCanvasContext);
  Array.from(function_array.values()).forEach((ctx: proceduralOffscreen) => {
    ctx.height = window.innerHeight;
    ctx.width = window.innerWidth;
    ctx.resetBitmap();
  });
  graph.resize(canvas);
  render();
});

render();
