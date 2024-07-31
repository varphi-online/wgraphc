import init, { draw_cnv, parse_text } from "../pkg/wgraphcal.js";
let initialized = false;
async function enssureWASMInit() {
    if (!initialized) {
        await init();
        initialized = true;
    }
}
export async function parse_string(string) {
    await enssureWASMInit();
    return await parse_text(string);
}
export async function oscDraw(offscreenCanvas, func, color, canvas, graph, ui) {
    await enssureWASMInit();
    //TODO: Replace some function inputs pretaining to fields in a
    // proceduralOffscreen into one and calling them from inside
    await draw_cnv(offscreenCanvas, canvas.width, canvas.height, graph.bounds[0], graph.bounds[1], graph.bounds[2], graph.bounds[3], ui.horizontal_axis, ui.vertical_axis, ui.slice, BigInt(ui.resolution), func, color, ui.continuity);
}
