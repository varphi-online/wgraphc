use crate::util::*;
use wasm_bindgen::prelude::*;

const GRAPH_RESOLUTION: i64 = 100;
const BUF_SIZE: usize = ((4 * GRAPH_RESOLUTION.pow(2) - 2) * 8) as usize;
static mut BUFFER: [f64; BUF_SIZE] = [0.0; BUF_SIZE];

#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn faster_call(
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
    canvas_pixel_width: f64,
    canvas_pixel_height: f64,
    wasm_size_sscalc: bool,
    x_axis: String,
    y_axis: String,
) {
    let graph_width: f64 = x2 - x1;
    let graph_height: f64 = y2 - y1;
    clog!("w: {}, h: {}", canvas_pixel_width, canvas_pixel_height);

    // Calculate the "Scale factor" of the graph in the x/y direction
    // this allows to plot points stabilly as each point will be exactly
    // one sfx unit from the previous, starting from the leftmost unit who is
    // closest to the graph's lower boundary in that direction.
    //
    // Because we round to the largest power of two that should be displayed,
    // it is inevitable that extra calculation occurs when nearing the next
    // closest power of two, so a refactoring should take place that limits
    // the calculation of out of bound points.
    let sfx = (2.0f64).powf(graph_width.log2().floor()) / (GRAPH_RESOLUTION as f64 / 2.0);
    let low_x_snap_bound = (x1 / sfx).floor() * sfx;
    let sfy = (2.0f64).powf(graph_height.log2().floor()) / (GRAPH_RESOLUTION as f64 / 2.0);
    let low_y_snap_bound = (y1 / sfy).floor() * sfy;

    // Temporary function takes one value in and applies some kind of math
    // transformation, in the future should take a complex64 or something
    // like it (simd and vector-based) and output a similar complex64-like
    fn apply_transformation(inp: f64) -> f64 {
        inp * inp * inp.sin()
    }

    // Remaps real and imaginary x,y coordinates to their pixel location
    // in screenspace, first by normalizing the coordinates to their pos
    // within the view bounds and then multiplying by the canvas pixel sizes
    let to_screenspace = |x: f64, y: f64| -> [f64; 2] {
        let normalized_real = 1.0 - (x2 - x) / (graph_width);
        let normalized_imaginary = (y2 - y) / (graph_height);
        [
            normalized_real * canvas_pixel_width,
            normalized_imaginary * canvas_pixel_height,
        ]
    };
    unsafe {
        // BUFFER is an allocation of memory equal to: 4*GRAPH_RESOLUTION^2 - 2
        // This represents an array, of groups of 4, of float64s representing some:
        // [input X/Real, input Y/Imaginary, output X/Real, output Y/Imaginary]
        let mut i = 0;
        for y in (0..=GRAPH_RESOLUTION).rev() {
            let coord_y: f64 = y as f64 * sfy + low_y_snap_bound;
            for x in 0..=GRAPH_RESOLUTION {
                let coord_x: f64 = x as f64 * sfx + low_x_snap_bound;
                if (i < BUF_SIZE) && i % 4 == 0 {
                    if wasm_size_sscalc {
                        BUFFER[i] = coord_x;
                        BUFFER[i + 1] = coord_y;
                        match (x_axis.as_str(), y_axis.as_str()) {
                            ("r", "r") => {
                                BUFFER[i + 2] = apply_transformation(BUFFER[i]);
                                [BUFFER[i], BUFFER[i + 2]] =
                                    to_screenspace(BUFFER[i], BUFFER[i + 2]);
                                BUFFER[i + 3] = apply_transformation(BUFFER[i]);
                                [BUFFER[i + 1], BUFFER[i + 3]] =
                                    to_screenspace(BUFFER[i + 1], BUFFER[i + 3]);
                            }
                            ("r", "i") => {}
                            ("i", "i") => {}
                            ("i", "r") => {}
                            _ => (),
                        }
                    } else {
                        BUFFER[i] = coord_x;
                        BUFFER[i + 1] = coord_y;
                        BUFFER[i + 2] = apply_transformation(BUFFER[i]);
                        BUFFER[i + 3] = apply_transformation(BUFFER[i + 1]);
                    }
                }
                i += 4;
            }
        }
    }
}

#[wasm_bindgen]
pub fn get_buf_as_ptr() -> *const f64 {
    let pointer: *const f64;
    unsafe {
        pointer = BUFFER.as_ptr();
    }
    pointer
}

#[wasm_bindgen]
pub fn get_resolution() -> i64 {
    GRAPH_RESOLUTION
}
