use super::super::parser::structs::{operator::Operator,function::Function};
use num_complex::*;
use wasm_bindgen::prelude::*;
use web_sys::OffscreenCanvasRenderingContext2d;

const GRAPH_RESOLUTION: i64 = 40;

#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn draw_cnv(
    ctx: &OffscreenCanvasRenderingContext2d,
    func: String,
    color: String,
    canvas_pixel_width: f64,
    canvas_pixel_height: f64,
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
    x_axis: String,
    y_axis: String,
    slice: f64,
    resolution: i64,
    continuity: bool,
    vars: String,
) {
    ctx.clear_rect(0.0, 0.0, canvas_pixel_width, canvas_pixel_height);
    let graph_width: f64 = x2 - x1;
    let graph_height: f64 = y2 - y1;

    /*
    Calculate the "Scale factor" of the graph in the x/y direction
    this allows to plot points stabilly as each point will be exactly
    one sfx unit from the previous, starting from the leftmost unit who is
    closest to the graph's lower boundary in that direction.

    Because we round to the largest power of two that should be displayed,
    it is inevitable that extra calculation occurs when nearing the next
    closest power of two, so a refactoring should take place that limits
    the calculation of out of bound points.
    */
    let sfx = (2.0f64).powf(graph_width.log2().floor())
        / ((resolution as f64 * GRAPH_RESOLUTION as f64) / 2.0);
    let low_x_snap_bound = (x1 / sfx).floor() * sfx;
    let sfy = (2.0f64).powf(graph_height.log2().floor())
        / ((resolution as f64 * GRAPH_RESOLUTION as f64) / 2.0);
    let low_y_snap_bound = (y1 / sfy).floor() * sfy;

    let ast: Option<Operator> = serde_json::from_str::<Operator>(&func).ok();

    let apply_transformation = |inp1: f64, inp2: f64| -> (f64, f64) {
        if let Some(value) = ast.clone() {
            let num = Complex64::new(inp1, inp2);
            let calc = value.eval(num, vars.clone());
            return (calc.re, calc.im);
        }
        (f64::INFINITY, f64::INFINITY)
    };

    /*
    Remaps real and imaginary x,y coordinates to their pixel location
    in screenspace, first by normalizing the coordinates to their pos
    within the view bounds and then multiplying by the canvas pixel sizes
    */
    let to_screenspace = |x: f64, y: f64| -> [f64; 2] {
        let normalized_horizontal = 1.0 - (x2 - x) / (graph_width);
        let normalized_vertical = (y2 - y) / (graph_height);
        [
            normalized_horizontal * canvas_pixel_width,
            normalized_vertical * canvas_pixel_height,
        ]
    };

    let mut todraw: Vec<[f64; 2]> = Vec::new();
    if x_axis.as_str() == "i_r" && y_axis.as_str() == "o_r" && slice == 0.0 {
        for value in 0..GRAPH_RESOLUTION * resolution {
            let num = value as f64 * sfx + low_x_snap_bound;
            let out = apply_transformation(num, 0.0);
            todraw.push(to_screenspace(num, out.0));
        }
    } else {
        for y in (0..GRAPH_RESOLUTION * resolution).rev() {
            let input_imag: f64 = y as f64 * sfy + low_y_snap_bound;
            for x in 0..(GRAPH_RESOLUTION * resolution) {
                let input_real: f64 = x as f64 * sfx + low_x_snap_bound;
                todraw.push(match (x_axis.as_str(), y_axis.as_str()) {
                    ("i_r", "o_r") => {
                        let (output_real, _output_imag) = apply_transformation(input_real, slice);
                        to_screenspace(input_real, output_real)
                    }
                    ("i_r", "i_i") => to_screenspace(input_real, input_imag),
                    ("i_r", "o_i") => {
                        let (_output_real, output_imag) =
                            apply_transformation(input_real, input_imag);
                        to_screenspace(input_real, output_imag)
                    }
                    ("o_r", "o_i") => {
                        let (output_real, output_imag) =
                            apply_transformation(input_real, input_imag);
                        to_screenspace(output_real, output_imag)
                    }
                    ("i_i", "o_i") => {
                        let (_utput_real, output_imag) = apply_transformation(slice, input_imag);
                        to_screenspace(input_imag, output_imag)
                    }
                    ("i_i", "o_r") => {
                        let (output_real, _output_imag) =
                            apply_transformation(input_real, input_imag);
                        to_screenspace(input_imag, output_real)
                    }
                    ("o_i", "o_r") => {
                        let (output_real, output_imag) =
                            apply_transformation(input_real, input_imag);
                        to_screenspace(output_imag, output_real)
                    }
                    _ => [0.0, 0.0],
                });
            }
        }
    }
    if continuity {
        /*
        TODO: Fix bug with the start and end of a quadratic curve and the
        imaginary input where the loop seems to close over the vector
        */
        ctx.set_stroke_style(&JsValue::from_str(color.as_str()));
        ctx.set_line_width(2.0);

        ctx.begin_path();
        ctx.move_to(todraw[0][0], todraw[0][1]);
        for i in 0..todraw.len() - 1 {
            let x_control = (todraw[i][0] + todraw[i + 1][0]) / 2.0;
            let y_control = (todraw[i][1] + todraw[i + 1][1]) / 2.0;
            ctx.quadratic_curve_to(
                todraw[i][0].round(),
                todraw[i][1].round(),
                x_control.round(),
                y_control.round(),
            );
        }

        ctx.stroke();
    } else {
        ctx.set_fill_style(&JsValue::from_str(color.as_str()));
        for i in todraw {
            ctx.fill_rect(i[0] - 1.0, i[1] - 1.0, 3.0, 3.0);
        }
    }
}