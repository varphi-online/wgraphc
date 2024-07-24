/* tslint:disable */
/* eslint-disable */
/**
* @param {string} input
* @returns {string}
*/
export function str_to_lexemes(input: string): string;
/**
* @param {string} input
* @returns {string}
*/
export function str_to_tokens(input: string): string;
/**
* @param {string} input
* @returns {string}
*/
export function str_to_abstract(input: string): string;
/**
* @param {string} input
* @returns {string}
*/
export function return_string(input: string): string;
/**
* @param {number} x1
* @param {number} x2
* @param {number} y1
* @param {number} y2
* @param {number} canvas_pixel_width
* @param {number} canvas_pixel_height
* @param {string} x_axis
* @param {string} y_axis
* @param {boolean} calculate_screensapce
* @param {number} slice
*/
export function faster_call(x1: number, x2: number, y1: number, y2: number, canvas_pixel_width: number, canvas_pixel_height: number, x_axis: string, y_axis: string, calculate_screensapce: boolean, slice: number): void;
/**
* @returns {number}
*/
export function get_buf_as_ptr(): number;
/**
* @returns {bigint}
*/
export function get_resolution(): bigint;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly str_to_lexemes: (a: number, b: number, c: number) => void;
  readonly str_to_tokens: (a: number, b: number, c: number) => void;
  readonly str_to_abstract: (a: number, b: number, c: number) => void;
  readonly return_string: (a: number, b: number, c: number) => void;
  readonly faster_call: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number) => void;
  readonly get_buf_as_ptr: () => number;
  readonly get_resolution: () => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
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
