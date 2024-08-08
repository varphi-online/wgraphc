use super::{super::parser::structs::operator::Operator, variables::str_to_varmap};
use num_complex::*;
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use web_sys::OffscreenCanvasRenderingContext2d;

const GRAPH_RESOLUTION: i64 = 40;

fn optimize_function(func: String, vars: String) -> Option<Operator> {
    // Here we parse two strings, combine and flatten them into one operator that can be evaluated
    if let Ok(function) = serde_json::from_str::<Operator>(&func) {
        Some(function.flatten(str_to_varmap(vars)))
    } else {
        None
    }
}

#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn draw_cnv(
    ctx: OffscreenCanvasRenderingContext2d,
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

    if let Some(ast) = optimize_function(func, vars) {
        let apply_transformation = |inp1: &f64, inp2: &f64| -> (f64, f64) {
            let num = Complex64::new(inp1.clone(), inp2.clone());
            let calc = ast.eval(num);
            return (calc.re, calc.im);
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

        //Return early if we only have to draw one point.
        if ast.is_constant() {
            let point: Complex64 = ast.eval(Complex64::new(0.0, 0.0));
            let [x, y] = match (x_axis.as_str(), y_axis.as_str()) {
                ("i_r", "o_r") => to_screenspace(point.re, 0.0),
                ("i_i", "o_i") => to_screenspace(0.0, point.im),
                _ => to_screenspace(
                    match x_axis.as_str() {
                        "i_r" | "o_r" => point.re,
                        "i_i" | "o_i" => point.im,
                        _ => 0.0,
                    },
                    match y_axis.as_str() {
                        "i_r" | "o_r" => point.re,
                        "i_i" | "o_i" => point.im,
                        _ => 0.0,
                    },
                ),
            };
            ctx.begin_path();
            let _circle = ctx.arc(x, y, 4.0, 0.0, PI * 2.0);
            ctx.set_fill_style(&JsValue::from_str(color.as_str()));
            ctx.fill();
            return;
        }

        let mut todraw: Vec<[f64; 2]> = Vec::new();
        if x_axis.as_str() == "i_r" && y_axis.as_str() == "o_r" && slice == 0.0 {
            for value in 0..GRAPH_RESOLUTION * resolution {
                let num = value as f64 * sfx + low_x_snap_bound;
                let out = apply_transformation(&num, &0.0);
                todraw.push(to_screenspace(num, out.0));
            }
        } else {
            for y in (0..GRAPH_RESOLUTION * resolution).rev() {
                let input_imag: f64 = y as f64 * sfy + low_y_snap_bound;
                for x in 0..(GRAPH_RESOLUTION * resolution) {
                    let input_real: f64 = x as f64 * sfx + low_x_snap_bound;
                    todraw.push(match (x_axis.as_str(), y_axis.as_str()) {
                        ("i_r", "o_r") => {
                            let (output_real, _output_imag) =
                                apply_transformation(&input_real, &slice);
                            to_screenspace(input_real, output_real)
                        }
                        ("i_r", "i_i") => to_screenspace(input_real, input_imag),
                        ("i_r", "o_i") => {
                            let (_output_real, output_imag) =
                                apply_transformation(&input_real, &input_imag);
                            to_screenspace(input_real, output_imag)
                        }
                        ("o_r", "o_i") => {
                            let (output_real, output_imag) =
                                apply_transformation(&input_real, &input_imag);
                            to_screenspace(output_real, output_imag)
                        }
                        ("i_i", "o_i") => {
                            let (_utput_real, output_imag) =
                                apply_transformation(&slice, &input_imag);
                            to_screenspace(input_imag, output_imag)
                        }
                        ("i_i", "o_r") => {
                            let (output_real, _output_imag) =
                                apply_transformation(&input_real, &input_imag);
                            to_screenspace(input_imag, output_real)
                        }
                        ("o_i", "o_r") => {
                            let (output_real, output_imag) =
                                apply_transformation(&input_real, &input_imag);
                            to_screenspace(output_imag, output_real)
                        }
                        _ => [0.0, 0.0],
                    });
                }
            }
        }
        draw_to_context(todraw, continuity, ctx, color);
    }
}

fn draw_to_context(
    points: Vec<[f64; 2]>,
    continuity: bool,
    context: OffscreenCanvasRenderingContext2d,
    color: String,
) {
    if continuity {
        //BUG: Fix bug with the start and end of a quadratic curve and the
        //imaginary input where the loop seems to close over the vector

        context.set_stroke_style(&JsValue::from_str(color.as_str()));
        context.set_line_width(2.0);

        context.begin_path();
        context.move_to(points[0][0], points[0][1]);
        for i in 0..points.len() - 1 {
            let x_control = (points[i][0] + points[i + 1][0]) / 2.0;
            let y_control = (points[i][1] + points[i + 1][1]) / 2.0;
            context.quadratic_curve_to(
                points[i][0].round(),
                points[i][1].round(),
                x_control.round(),
                y_control.round(),
            );
        }

        context.stroke();
    } else {
        context.set_fill_style(&JsValue::from_str(color.as_str()));
        for i in points {
            context.fill_rect(i[0] - 1.0, i[1] - 1.0, 3.0, 3.0);
        }
    }
}
