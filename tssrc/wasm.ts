import init, {
	draw_cnv,
	parse_text,
	set_var,
	del_var,
	debug,
} from "../pkg/wgraphcal.js";
import {
	canvasContainer,
	graphContainer,
	proceduralOffscreen,
} from "./graph.js";
import { metaUIContainer } from "./metaUI.js";

let initialized = false;

async function enssureWASMInit() {
	if (!initialized) {
		await init();
		initialized = true;
	}
}

export async function get_wasm_debug(): Promise<boolean> {
	await enssureWASMInit();
	return await debug();
}

export async function parse_string(string: string) {
	await enssureWASMInit();
	return await parse_text(string);
}

export async function set_variable(
	key: string,
	val: string,
	map: string,
): Promise<string> {
	return await set_var(key, val, map);
}

export async function del_variable(key: string, map: string): Promise<string> {
	return await del_var(key, map);
}

export async function oscDraw(
	ctx: proceduralOffscreen,
	canvas: canvasContainer,
	graph: graphContainer,
	ui: metaUIContainer,
	vars: string,
) {
	await enssureWASMInit();
	if (ctx.draw) {
		await draw_cnv(
			ctx.context,
			ctx.serialized_function,
			ctx.color,
			canvas.width,
			canvas.height,
			graph.bounds[0],
			graph.bounds[1],
			graph.bounds[2],
			graph.bounds[3],
			ui.horizontal_axis,
			ui.vertical_axis,
			ui.slice,
			BigInt(ui.resolution),
			ui.continuity,
			vars,
		);
	} else {
		ctx.context.clearRect(0, 0, canvas.width, canvas.height);
	}
}
