import { canvasContainer, graphContainer, } from "./graph.js";
import { function_text_inputs } from "./metaUI.js";
let canvas = new canvasContainer();
canvas.init();
// Initialize the GL context
export const mainCanvasContext = canvas.object.getContext("2d");
canvas.resetBitmap(mainCanvasContext);
mainCanvasContext.font = "15px serif";
mainCanvasContext.strokeStyle = "white";
mainCanvasContext.lineWidth = 3;
mainCanvasContext.textAlign = "center";
// Initialize global graph obj
export let graph = new graphContainer(canvas);
graph.init();
// Map of offscreen canvases to be drawn to
let function_array = new Map();
// Object of inputs to build above map
let inputs = new function_text_inputs();
inputs.init(inputs, function_array);
canvas.event_init(canvas, graph);
export async function render() {
    await graph.render(canvas, mainCanvasContext, graph, Array.from(function_array.values()));
}
render();
