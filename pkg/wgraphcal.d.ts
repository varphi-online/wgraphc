/* tslint:disable */
/* eslint-disable */
/**
* @param {OffscreenCanvasRenderingContext2D} ctx
* @param {string} func
* @param {string} color
* @param {number} canvas_pixel_width
* @param {number} canvas_pixel_height
* @param {number} x1
* @param {number} x2
* @param {number} y1
* @param {number} y2
* @param {string} x_axis
* @param {string} y_axis
* @param {number} slice
* @param {bigint} resolution
* @param {boolean} continuity
* @param {string} vars
*/
export function draw_cnv(ctx: OffscreenCanvasRenderingContext2D, func: string, color: string, canvas_pixel_width: number, canvas_pixel_height: number, x1: number, x2: number, y1: number, y2: number, x_axis: string, y_axis: string, slice: number, resolution: bigint, continuity: boolean, vars: string): void;
/**
* @param {string} key
* @param {string} value
* @param {string} map
* @returns {string}
*/
export function set_var(key: string, value: string, map: string): string;
/**
* @param {string} key
* @param {string} map
* @returns {string}
*/
export function del_var(key: string, map: string): string;
/**
* @param {number} real
* @param {number} imag
* @returns {string}
*/
export function number_operator_from_2df64(real: number, imag: number): string;
/**
* @param {string} input
* @param {string} map
* @returns {string}
*/
export function faster_parse_input(input: string, map: string): string;
/**
* @returns {boolean}
*/
export function debug(): boolean;
/**
* @param {string} input
* @returns {string}
*/
export function parse_text(input: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly draw_cnv: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number, r: number, s: number, t: number) => void;
  readonly set_var: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly del_var: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly number_operator_from_2df64: (a: number, b: number, c: number) => void;
  readonly faster_parse_input: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly debug: () => number;
  readonly parse_text: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
