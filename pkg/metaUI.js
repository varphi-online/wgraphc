import { graph, mainCanvasContext, render } from "./index.js";
import { proceduralOffscreen } from "./graph.js";
import { parse_string } from "./wasm.js";
export class function_text_inputs {
    elements;
    element_ids;
    container;
    id_incr;
    constructor() {
        this.id_incr = 0;
        this.elements = Array.from(document.getElementsByClassName("funcInput"));
        this.container = document.getElementById("inputs");
        this.element_ids = new Map();
    }
    init(self, contextMap) {
        for (let [index, element] of this.elements.entries()) {
            let id_increment = JSON.stringify(this.id_incr);
            this.element_ids.set(element.id, this.elements.length - 1);
            contextMap.set(id_increment, new proceduralOffscreen());
            let initial = contextMap.get(id_increment);
            if (initial) {
                initial.color = "blue";
            }
            this.attach_oninput(element, contextMap, id_increment);
            this.attach_event_listener(element, self, contextMap);
            this.increment_id();
        }
    }
    increment_id() {
        this.id_incr++;
    }
    // Unique oninput for each function allows for different ones to be drawn
    attach_oninput(elem, contextMap, id) {
        elem.oninput = async function () {
            let context = contextMap.get(id);
            if (context) {
                context.serialized_function = await parse_string(elem.value);
            }
            await render();
        };
    }
    attach_event_listener(elem, self, contextMap) {
        // On enter, creates a new input element with it's own offscreen canvas
        // and appropriately updates the arrays including it
        elem.addEventListener("keydown", (event) => {
            if (event.key === "Enter") {
                let id_increment = JSON.stringify(self.id_incr);
                let new_input_box = document.createElement("input");
                contextMap.set(id_increment, new proceduralOffscreen());
                self.attach_event_listener(new_input_box, self, contextMap);
                new_input_box.className = "funcInput";
                new_input_box.id = "funcInput-" + parseInt(id_increment);
                self.container.appendChild(new_input_box);
                self.elements.push(new_input_box);
                self.element_ids.set(new_input_box.id, self.elements.length - 1);
                this.attach_oninput(new_input_box, contextMap, id_increment);
                new_input_box.focus();
                self.increment_id();
            }
            // On backspace, if the box is empty, destroys itsself after removing
            // itsself from all info arrays and setting focus to previous box
            if (event.key === "Backspace" && elem.value == "") {
                if (self.elements.length > 1) {
                    let idx = elem.id.replace("funcInput-", "");
                    contextMap.delete(idx);
                    let index_of_remove = self.element_ids.get(elem.id);
                    self.element_ids.delete(elem.id);
                    let to_remove = document.getElementById(elem.id);
                    if (to_remove) {
                        self.container.removeChild(to_remove);
                    }
                    if (index_of_remove) {
                        self.elements[index_of_remove - 1].focus();
                        self.elements.splice(index_of_remove, 1);
                    }
                }
            }
        });
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
                metaUIobj.slice_slider.setAttribute("step", String((parseFloat(metaUIobj.slice_max_input.value) -
                    parseFloat(metaUIobj.slice_min_input.value) / 2) /
                    200));
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
