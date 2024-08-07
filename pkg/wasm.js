import init, { draw_cnv, parse_text, set_var, del_var, debug, number_operator_from_2df64, faster_parse_input } from "../pkg/wgraphcal.js";
let initialized = false;
async function ensureWASMInit() {
    if (!initialized) {
        await init();
        initialized = true;
    }
}
export async function faster_get_input_type(input, map) {
    await ensureWASMInit();
    return await faster_parse_input(input, map);
}
export async function get_num_op(real, imag) {
    await ensureWASMInit();
    return await number_operator_from_2df64(real, imag);
}
export async function get_wasm_debug() {
    await ensureWASMInit();
    return await debug();
}
export async function parse_string(string) {
    await ensureWASMInit();
    return await parse_text(string);
}
export async function set_variable(key, val, map) {
    return await set_var(key, val, map);
}
export async function del_variable(key, map) {
    return await del_var(key, map);
}
export async function oscDraw(ctx, canvas, graph, ui, vars) {
    await ensureWASMInit();
    if (ctx.draw) {
        await draw_cnv(ctx.context, ctx.serialized_function, ctx.color, canvas.width, canvas.height, graph.bounds[0], graph.bounds[1], graph.bounds[2], graph.bounds[3], ui.horizontal_axis, ui.vertical_axis, ui.slice, BigInt(ui.resolution), ui.continuity, vars);
    }
    else {
        ctx.context.clearRect(0, 0, canvas.width, canvas.height);
    }
}
