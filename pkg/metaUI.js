import { graph, mainCanvasContext, render, var_map, set_varmap, } from "./index.js";
import { proceduralOffscreen } from "./graph.js";
import { set_variable, del_variable, get_num_op, faster_get_input_type, } from "./wasm.js";
let recursion_limit = 1;
class function_box {
    /*
      A function box object describes a div xontaining the textual input field
      in HTML, optionally a slider input the offscreen context attached to it,
      and a few other attributes useful in working with the data stored in
      itsself.
  
      On "Enter", the function box creates a new input box with its own constructor
      and on "Backspace" deletes itsself, all mappings/references to itsself and
      moves the user's cursor focus to the input box directly above.
  
      If a box contains a variable declaration (i.e. <variableName>=<number>) it
      will show a slider element to allow for ease of variable changing, and
      will hide said slider if the declaration is invalid/non-exisent
  
      This also acts as a node in a reverse linked list for easy access of the previous
      */
    container;
    text_box;
    text_box_evaluated;
    slider_container;
    slider_real_container;
    slider_real;
    slider_real_min;
    slider_real_max;
    slider_imag_container;
    slider_imag;
    slider_imag_min;
    slider_imag_max;
    // Keeps track of if a var decl. is in progress to not reset slider
    decl_flag;
    offscreen;
    index;
    variable_name;
    mapped_var_name;
    variable_value;
    variable_type;
    parent;
    context_map;
    next;
    previous;
    dependencies;
    constructor(parent, contextMap) {
        // Create the DOM elements that define this object
        // Index will be relative to place in containing array
        this.context_map = contextMap;
        this.parent = parent;
        this.index = this.parent.function_boxes.length;
        this.offscreen = new proceduralOffscreen();
        contextMap.set(this.index, this.offscreen);
        this.initialize_DOM();
        this.variable_name = "";
        this.mapped_var_name = "";
        this.variable_value = [0, 0];
        this.variable_type = "";
        this.dependencies = [];
        this.initialize_Inputs();
        this.parent.function_boxes.push(this);
    }
    initialize_DOM() {
        // Prodceedurally create an element from some key info and cast back to type
        const createElement = (tag, classes, id_suffix, input_type) => {
            let element = document.createElement(tag);
            element.id = "function_box_" + id_suffix + "-" + String(this.index);
            classes.unshift(id_suffix);
            element.className = "function_box_" + classes.join(" ");
            if (element instanceof HTMLInputElement) {
                if (input_type == "range") {
                    element.type = input_type;
                    element.min = "-10";
                    element.max = "10";
                }
                element.autocomplete = "off";
            }
            return element;
        };
        //TODO: Add color selector and play button for func box
        let container = createElement("div", [], "container", null);
        let input = (createElement("input", ["function_box_text_input"], "input", null));
        // The slider is hidden by default and only is visible with a var decl.
        let slider_container = (createElement("div", [], "slider_container", null));
        slider_container.style.display = "none";
        slider_container.style.flexDirection = "column";
        let slider_real_container = (createElement("div", ["function_box_slider_container"], "slider_real_container", null));
        let slider_real = (createElement("input", ["function_box_slider"], "slider_real", "range"));
        let slider_real_min = (createElement("input", ["function_box_slider_input", "function_box_input", "left"], "slider_real_min", null));
        let slider_real_max = (createElement("input", ["function_box_slider_input", "function_box_input", "right"], "slider_real_max", null));
        let slider_imag_container = (createElement("div", ["function_box_slider_container"], "slider_imag_container", null));
        let slider_imag = (createElement("input", ["function_box_slider"], "slider_imag", "range"));
        let slider_imag_min = (createElement("input", ["function_box_slider_input", "function_box_input", "left"], "slider_imag_min", null));
        let slider_imag_max = (createElement("input", ["function_box_slider_input", "function_box_input", "right"], "slider_imag_max", null));
        let evaluated = (createElement("p", [], "input_evaluated", null));
        slider_real_container.appendChild(slider_real_min);
        slider_real_container.appendChild(slider_real);
        slider_real_container.appendChild(slider_real_max);
        slider_imag_container.appendChild(slider_imag_min);
        slider_imag_container.appendChild(slider_imag);
        slider_imag_container.appendChild(slider_imag_max);
        slider_container.appendChild(slider_real_container);
        slider_container.appendChild(slider_imag_container);
        container.appendChild(input);
        container.appendChild(slider_container);
        container.appendChild(evaluated);
        this.container = container;
        this.text_box = input;
        this.slider_container = slider_container;
        this.slider_real_container = slider_real_container;
        this.slider_real = slider_real;
        this.slider_real_min = slider_real_min;
        this.slider_real_max = slider_real_max;
        this.slider_imag_container = slider_imag_container;
        this.slider_imag = slider_imag;
        this.slider_imag_min = slider_imag_min;
        this.slider_imag_max = slider_imag_max;
        this.text_box_evaluated = evaluated;
    }
    initialize_Inputs() {
        let self = this;
        // Primary textual input
        this.text_box.oninput = async function () {
            await self.handle_string(self.text_box.value, recursion_limit);
            function set_min_max_val(min_box, slider, max_box, min, max, value) {
                min_box.value = String(min);
                max_box.value = String(max);
                slider.min = String(min);
                slider.value = String(value);
                slider.max = String(max);
                slider.step = String(Math.abs((max - min) / 200));
            }
            if (self.variable_name != "") {
                self.parent.name_map.delete(self.mapped_var_name);
                self.mapped_var_name = self.variable_name;
                self.parent.name_map.add(self.mapped_var_name);
                self.slider_container.style.display = "flex";
                switch (self.variable_type) {
                    case "000":
                        self.slider_real_container.style.display = "flex";
                        self.slider_imag_container.style.display = "none";
                        break;
                    case "001":
                        self.slider_real_container.style.display = "none";
                        self.slider_imag_container.style.display = "flex";
                        break;
                    case "010":
                        self.slider_real_container.style.display = "flex";
                        self.slider_imag_container.style.display = "flex";
                        break;
                    default:
                        self.slider_container.style.display = "none";
                        break;
                }
                if (!self.decl_flag) {
                    set_min_max_val(self.slider_real_min, self.slider_real, self.slider_real_max, self.variable_value[0] - 10, self.variable_value[0] + 10, self.variable_value[0]);
                    set_min_max_val(self.slider_imag_min, self.slider_imag, self.slider_imag_max, self.variable_value[1] - 10, self.variable_value[1] + 10, self.variable_value[1]);
                }
                // Handles changing of the value if the slider has been used
                else {
                    // Specific cases where variable set is out of defined range
                    if (parseFloat(self.slider_real_min.value) > self.variable_value[0]) {
                        set_min_max_val(self.slider_real_min, self.slider_real, self.slider_real_max, self.variable_value[0], parseFloat(self.slider_real_max.value), self.variable_value[0]);
                    }
                    else if (parseFloat(self.slider_real_max.value) < self.variable_value[0]) {
                        set_min_max_val(self.slider_real_min, self.slider_real, self.slider_real_max, parseFloat(self.slider_real_min.value), self.variable_value[0], self.variable_value[0]);
                    }
                    else {
                        self.slider_real.value = String(self.variable_value[0]);
                    }
                    if (parseFloat(self.slider_imag_min.value) > self.variable_value[1]) {
                        set_min_max_val(self.slider_imag_min, self.slider_imag, self.slider_imag_max, self.variable_value[0], parseFloat(self.slider_imag_max.value), self.variable_value[0]);
                    }
                    else if (parseFloat(self.slider_imag_max.value) < self.variable_value[1]) {
                        set_min_max_val(self.slider_imag_min, self.slider_imag, self.slider_imag_max, parseFloat(self.slider_imag_min.value), self.variable_value[0], self.variable_value[0]);
                    }
                    else {
                        self.slider_imag.value = String(self.variable_value[1]);
                    }
                }
            }
            else {
                self.slider_container.style.display = "none";
            }
            await render();
        };
        // Handles specifically the key pressing of Enter and Backspace
        this.text_box.addEventListener("keydown", async (event) => {
            // On Enter, creates a new input element with it's own offscreen canvas
            if (event.key === "Enter") {
                let new_function_box = new function_box(self.parent, self.context_map);
                // Update internal linked list
                new_function_box.previous = self;
                new_function_box.next = self.next;
                self.next = new_function_box;
                self.container.insertAdjacentElement("afterend", new_function_box.container);
                new_function_box.text_box.focus();
            }
            // On Backspace, if the box is empty, destroys itsself after removing
            // itsself from all info arrays and setting focus to previous box
            else if (event.key === "Backspace" &&
                self.text_box.value == "" &&
                self.previous) {
                // Update mappings
                self.context_map.delete(self.index);
                set_varmap(await del_variable(self.mapped_var_name, var_map));
                self.previous.text_box.focus();
                self.previous.next = self.next;
                self.parent.container.removeChild(self.container);
            }
        });
        // Slider changes will be reflected in the varmap and the sister text-box
        this.slider_real.oninput = async function () {
            self.decl_flag = true;
            self.variable_value[0] = parseFloat(self.slider_real.value);
            let [value, display] = (await get_num_op(self.variable_value[0], self.variable_value[1])).split("~");
            set_varmap(await set_variable(self.variable_name, value, var_map));
            self.text_box.value = self.variable_name + "=" + display;
            self.parent.function_boxes.forEach(box => { box.update(recursion_limit); });
            if (self.text_box_evaluated.innerText.length > 1) {
                let [r, i] = self.variable_value;
                self.text_box_evaluated.innerText = r.toPrecision(4) + "+" + i.toPrecision(4) + "i";
            }
            await render();
        };
        this.slider_imag.oninput = async function () {
            self.decl_flag = true;
            self.variable_value[1] = parseFloat(self.slider_imag.value);
            let [value, display] = (await get_num_op(self.variable_value[0], self.variable_value[1])).split("~");
            set_varmap(await set_variable(self.variable_name, value, var_map));
            self.text_box.value = self.variable_name + "=" + display;
            if (self.text_box_evaluated.innerText.length > 1) {
                let [r, i] = self.variable_value;
                self.text_box_evaluated.innerText = r.toPrecision(4) + "+" + i.toPrecision(4) + "i";
            }
            self.parent.function_boxes.forEach(box => { box.update(recursion_limit); });
            await render();
        };
        // Update slider bounds and step on input
        this.slider_real_min.oninput = async function () {
            self.slider_real.min = self.slider_real_min.value;
            self.slider_real.step = String(Math.abs(parseFloat(self.slider_real.min) - parseFloat(self.slider_real.max)) / 200);
        };
        this.slider_real_max.oninput = async function () {
            self.slider_real.max = self.slider_real_max.value;
            self.slider_real.step = String(Math.abs(parseFloat(self.slider_real.min) - parseFloat(self.slider_real.max)) / 200);
        };
        this.slider_imag_min.oninput = async function () {
            self.slider_imag.min = self.slider_imag_min.value;
            self.slider_imag.step = String(Math.abs(parseFloat(self.slider_real.min) - parseFloat(self.slider_real.max)) / 200);
        };
        this.slider_imag_max.oninput = async function () {
            self.slider_imag.max = self.slider_imag_max.value;
            self.slider_imag.step = String(Math.abs(parseFloat(self.slider_real.min) - parseFloat(self.slider_real.max)) / 200);
        };
    }
    async handle_string(input, update) {
        //Check if valid variable assignment, else, try to parse as expression
        this.text_box_evaluated.innerText = "";
        if (input.includes("=")) {
            let [id, arg] = input.split("=", 2);
            let [num_type, expression, value, deps] = (await faster_get_input_type(arg, var_map)).split("~", 4);
            if (id.includes("(") && /(([a-zA-Z]+)(_(\{(\w*?(})?)?)?)?)\((,?(([a-zA-Z]+)(_(\{(\w*?(})?)?)?)?))*?\)/.test(id)) {
                // Case of function
                this.offscreen.serialized_function = expression;
                this.offscreen.draw = true;
            }
            else if (/(([a-zA-Z]+)(_(\{(\w*?(})?)?)?)?)/.test(id)) {
                // Case of variable:
                set_varmap(await set_variable(id, expression, await del_variable(this.variable_name, var_map)));
                if (value.length > 1) {
                    let [r, i] = value.split(",");
                    this.text_box_evaluated.innerText = parseFloat(r).toPrecision(4) + "+" + parseFloat(i).toPrecision(4) + "i";
                }
                this.variable_type = num_type;
                this.offscreen.draw = false;
                this.variable_name = id;
                this.variable_value = value.split(",").map((val) => {
                    return parseFloat(val);
                });
            }
            else {
                // Case of malformed input
                set_varmap(await del_variable(this.variable_name, var_map));
                this.offscreen.set_draw(false);
                this.offscreen.serialized_function = "";
                this.variable_name = "";
            }
            try {
                this.dependencies = JSON.parse(deps);
            }
            catch {
                this.dependencies = [];
            }
            ;
        }
        else {
            let [num_type, expression, value, deps] = (await faster_get_input_type(input, var_map)).split("~", 4);
            // Case of expression
            this.offscreen.serialized_function = expression;
            this.offscreen.draw = true;
            if (value.length > 1) {
                let [r, i] = value.split(",");
                this.text_box_evaluated.innerText = parseFloat(r).toPrecision(4) + "+" + parseFloat(i).toPrecision(4) + "i";
            }
            try {
                this.dependencies = JSON.parse(deps);
            }
            catch {
                this.dependencies = [];
            }
            ;
        }
        if (update > 0)
            this.parent.function_boxes.forEach(box => { box.update(update); });
    }
    update(recursion_lifetime) {
        const { name_map } = this.parent;
        if (this.dependencies.some(dep => name_map.has(dep))) {
            this.handle_string(this.text_box.value, recursion_lifetime - 1);
        }
    }
}
export class function_text_inputs {
    function_boxes;
    container;
    name_map;
    context_map;
    constructor(map) {
        this.function_boxes = [];
        this.container = document.getElementById("inputs");
        this.context_map = map;
        this.name_map = new Set();
        let first_box = new function_box(this, map);
        this.function_boxes.push(first_box);
        this.container.appendChild(this.function_boxes[0].container);
    }
}
export class metaUIContainer {
    horizontal_axis_selector;
    vertical_axis_selector;
    horizontal_axis;
    vertical_axis;
    reset_view_button;
    frame_time;
    continuity_toggle;
    continuity;
    bounds_inputs;
    resolution_input;
    resolution_up_stepper;
    resolution_down_stepper;
    resolution;
    slice_slider;
    slice;
    slice_min_input;
    slice_max_input;
    slice_input;
    constructor() {
        // User to select which axes are rendered
        this.horizontal_axis_selector = (document.getElementById("haxis"));
        this.vertical_axis_selector = (document.getElementById("vaxis"));
        this.horizontal_axis = "i_r";
        this.vertical_axis = "o_r";
        this.reset_view_button = (document.getElementById("resetView"));
        this.frame_time = (document.getElementById("frameTime"));
        this.continuity_toggle = document.getElementById("cont");
        this.continuity = true;
        this.bounds_inputs = [
            document.getElementById("x1"),
            document.getElementById("x2"),
            document.getElementById("y1"),
            document.getElementById("y2"),
        ];
        this.resolution_input = (document.getElementById("resolution"));
        this.resolution_up_stepper = (document.getElementById("rezUp"));
        this.resolution_down_stepper = (document.getElementById("rezDown"));
        this.resolution = 3;
        this.slice_slider = document.getElementById("slice");
        this.slice = 0;
        this.slice_min_input = (document.getElementById("minSlice"));
        this.slice_max_input = (document.getElementById("maxSlice"));
        this.slice_input = document.getElementById("sliceVal");
        init_slice(this);
        init_axes(this);
        init_event_listeners(this);
    }
    set_bounds_inputs(index, value) {
        if (this.bounds_inputs[index]) {
            this.bounds_inputs[index].value = String(value);
        }
    }
}
function init_slice(metaUIobj) {
    function update_slice_range() {
        if (metaUIobj.slice_slider &&
            metaUIobj.slice_min_input &&
            metaUIobj.slice_max_input) {
            if (metaUIobj.slice_min_input && metaUIobj.slice_max_input) {
                metaUIobj.slice_slider.setAttribute("min", String(parseFloat(metaUIobj.slice_min_input.value)));
                metaUIobj.slice_slider.setAttribute("max", String(parseFloat(metaUIobj.slice_max_input.value)));
                metaUIobj.slice_slider.setAttribute("step", "any"
                //String(
                //  (parseFloat(metaUIobj.slice_max_input.value) -
                //    parseFloat(metaUIobj.slice_min_input.value) / 2) /
                //    200
                //)
                );
            }
        }
    }
    window.updateRange = update_slice_range;
    if (metaUIobj.slice_slider &&
        metaUIobj.slice_min_input &&
        metaUIobj.slice_max_input) {
        metaUIobj.slice_slider.oninput = async function () {
            if (metaUIobj.slice_slider &&
                metaUIobj.slice_min_input &&
                metaUIobj.slice_max_input) {
                let val = (document.getElementById("sliceVal"));
                if (val) {
                    val.value = metaUIobj.slice_slider.value;
                    metaUIobj.slice = parseFloat(metaUIobj.slice_slider.value);
                    await render();
                }
            }
        };
        if (metaUIobj.slice_input) {
            metaUIobj.slice_input.oninput = async function () {
                if (metaUIobj.slice_input && metaUIobj.slice_slider) {
                    metaUIobj.slice_slider.value = String(parseFloat(metaUIobj.slice_input.value));
                    metaUIobj.slice = parseFloat(metaUIobj.slice_input.value);
                    await render();
                }
            };
        }
    }
}
function init_axes(metaUIobj) {
    async function on_axis_change() {
        if (metaUIobj.horizontal_axis_selector &&
            metaUIobj.vertical_axis_selector) {
            metaUIobj.horizontal_axis = metaUIobj.horizontal_axis_selector.value;
            metaUIobj.vertical_axis = metaUIobj.vertical_axis_selector.value;
        }
        render();
    }
    if (metaUIobj.horizontal_axis_selector && metaUIobj.vertical_axis_selector) {
        metaUIobj.horizontal_axis_selector.onchange = on_axis_change;
        metaUIobj.vertical_axis_selector.onchange = on_axis_change;
    }
}
function init_event_listeners(metaUIobj) {
    if (metaUIobj.reset_view_button) {
        metaUIobj.reset_view_button.addEventListener("click", async function () {
            graph.reset(mainCanvasContext);
            graph.updateBounds();
            await render();
        });
    }
    if (metaUIobj.continuity_toggle) {
        metaUIobj.continuity_toggle.addEventListener("click", async function () {
            metaUIobj.continuity = !metaUIobj.continuity;
            await render();
        });
    }
    if (metaUIobj.bounds_inputs) {
        for (let [index, val] of metaUIobj.bounds_inputs.entries()) {
            if (val) {
                val.oninput = async function () {
                    if (!isNaN(parseFloat(val.value))) {
                        graph.initialBounds[index] = parseFloat(val.value);
                        if (!val.value.endsWith(".")) {
                            graph.updateBounds();
                            await render();
                        }
                    }
                };
            }
        }
    }
    if (metaUIobj.resolution_input) {
        metaUIobj.resolution_input.addEventListener("change", async function () {
            metaUIobj.resolution = parseInt(metaUIobj.resolution_input.value);
            await render();
        });
        if (metaUIobj.resolution_up_stepper) {
            metaUIobj.resolution_up_stepper.addEventListener("click", async function () {
                metaUIobj.resolution = parseInt(metaUIobj.resolution_input.value);
                await render();
            });
        }
        if (metaUIobj.resolution_down_stepper) {
            metaUIobj.resolution_down_stepper.addEventListener("click", async function () {
                metaUIobj.resolution = parseInt(metaUIobj.resolution_input.value);
                await render();
            });
        }
    }
    if (metaUIobj.slice_slider) {
        metaUIobj.slice_slider.addEventListener("change", async function () {
            metaUIobj.slice = parseFloat(metaUIobj.slice_slider.value);
            await render();
        });
    }
}
