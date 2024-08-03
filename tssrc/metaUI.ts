import {
	graph,
	mainCanvasContext,
	render,
	var_map,
	set_varmap,
} from "./index.js";
import { proceduralOffscreen } from "./graph.js";
import { parse_string, set_variable, del_variable } from "./wasm.js";

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
	public container: HTMLDivElement;
	public text_box: HTMLInputElement;
	private slider_container: HTMLDivElement;
	private slider: HTMLInputElement;
	private slider_min: HTMLInputElement;
	private slider_max: HTMLInputElement;

	// Keeps track of if a var decl. is in progress to not reset slider
	private decl_flag: boolean;
	public offscreen: proceduralOffscreen;
	public index: number;
	public variable_name: string;
	private mapped_var_name: string;
	public variable_value: number;
	public variable_imag: boolean;

	private parent: function_text_inputs;
	private context_map: Map<number, proceduralOffscreen>;
	public next: function_box;
	public previous: function_box;

	constructor(
		parent: function_text_inputs,
		contextMap: Map<number, proceduralOffscreen>,
	) {
		// Create the DOM elements that define this object
		// Index will be relative to place in containing array
		this.context_map = contextMap;
		this.parent = parent;
		this.index = this.parent.function_boxes.length;
		this.offscreen = new proceduralOffscreen();
		contextMap.set(this.index, this.offscreen);

		this.initialize_DOM();
		this.variable_name = "";
		this.variable_value = 0;
		this.variable_imag = false;

		this.initialize_Inputs();
		this.parent.function_boxes.push(this);
	}

	initialize_DOM() {
		// Prodceedurally create an element from some key info and cast back to type
		const createElement = (
			tag: string,
			classes: Array<String>,
			id_suffix: string,
			input_type: string | null,
		): HTMLElement => {
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

		let container = <HTMLDivElement>createElement("div", [], "container", null);
		let input = <HTMLInputElement>(
			createElement("input", ["function_box_text_input"], "input", null)
		);

		// The slider is hidden by default and only is visible with a var decl.
		let slider_container = <HTMLDivElement>(
			createElement("div", [], "slider_container", null)
		);
		slider_container.style.display = "none";
		let slider = <HTMLInputElement>(
			createElement("input", [], "slider", "range")
		);
		let slider_min = <HTMLInputElement>(
			createElement(
				"input",
				["function_box_slider_input", "function_box_input", "left"],
				"slider_min",
				null,
			)
		);
		let slider_max = <HTMLInputElement>(
			createElement(
				"input",
				["function_box_slider_input", "function_box_input", "right"],
				"slider_max",
				null,
			)
		);

		slider_container.appendChild(slider_min);
		slider_container.appendChild(slider);
		slider_container.appendChild(slider_max);
		container.appendChild(input);
		container.appendChild(slider_container);

		this.container = container;
		this.text_box = input;
		this.slider_container = slider_container;
		this.slider = slider;
		this.slider_min = slider_min;
		this.slider_max = slider_max;
	}

	initialize_Inputs() {
		let self = this;

		// Primary textual input
		this.text_box.oninput = async function() {
			[self.variable_name, self.variable_value, self.variable_imag] =
				await self.handle_string(self.text_box.value, self.offscreen);

			if (self.variable_name != "") {
				self.mapped_var_name = self.variable_name;
				// We don't want to waste extra time drawing to the screen if
				// the box declares a var
				self.offscreen.draw = false;
				self.slider_container.style.display = "flex";
				if (!self.decl_flag) {
					self.slider_min.value = String(self.variable_value - 10);
					self.slider.min = String(self.variable_value - 10);
					self.slider_max.value = String(self.variable_value + 10);
					self.slider.max = String(self.variable_value + 10);
					self.slider.value = String(self.variable_value);
					self.slider.step = String(self.variable_value / 200);
				} // Handles changing of the value if the slider has been used
				else {
					// Specific cases where variable set is out of defined range
					let value = String(self.variable_value);
					if (parseFloat(self.slider_min.value) > self.variable_value) {
						self.slider_min.value = value;
						self.slider.min = value;
						self.slider.value = value;
						self.slider.step = String(
							(parseFloat(self.slider.min) + parseFloat(self.slider.max)) /
							2 /
							200,
						);
					} else if (parseFloat(self.slider_max.value) < self.variable_value) {
						self.slider_max.value = value;
						self.slider.max = value;
						self.slider.value = value;
						self.slider.step = String(
							(parseFloat(self.slider.min) + parseFloat(self.slider.max)) /
							2 /
							200,
						);
					} else {
						self.slider.value = String(self.variable_value);
					}
				}
			} else {
				self.slider_container.style.display = "none";
				self.offscreen.draw = true;
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
				self.next = new_function_box;

				self.container.insertAdjacentElement(
					"afterend",
					new_function_box.container,
				);
				new_function_box.text_box.focus();
			}
			// On Backspace, if the box is empty, destroys itsself after removing
			// itsself from all info arrays and setting focus to previous box
			else if (
				event.key === "Backspace" &&
				self.text_box.value == "" &&
				self.previous
			) {
				// Update mappings
				self.context_map.delete(self.index);
				set_varmap(await del_variable(self.mapped_var_name, var_map));
				self.previous.text_box.focus();
				self.previous.next = self.next;
				self.parent.container.removeChild(self.container);
			}
		});

		// Slider changes will be reflected in the varmap and the sister text-box
		this.slider.oninput = async function() {
			self.decl_flag = true;
			self.variable_value = parseFloat(self.slider.value);
			set_varmap(
				await set_variable(
					self.variable_name,
					String(self.variable_value),
					var_map,
				),
			);
			self.text_box.value =
				self.variable_name + "=" + String(self.variable_value);
			await render();
		};

		// Update slider bounds and step on input
		this.slider_min.oninput = async function() {
			self.slider.min = self.slider_min.value;
			self.slider.step = String(
				(parseFloat(self.slider.min) + parseFloat(self.slider.max)) / 2 / 200,
			);
		};

		this.slider_max.oninput = async function() {
			self.slider.max = self.slider_max.value;
			self.slider.step = String(
				(parseFloat(self.slider.min) + parseFloat(self.slider.max)) / 2 / 200,
			);
		};
	}

	async handle_string(
		value: string,
		context: proceduralOffscreen,
	): Promise<[string, number, boolean]> {
		// Check if valid variable assignment, else, try to parse as expression
		if (value.includes("=")) {
			let split = value.split("=", 2);
			if (
				/([a-zA-Z]+)(_({(\w*(})?)?)?)?$/gy.test(split[0]) &&
				/(\d)+(\.)?(\d)*[i]?$/gy.test(split[1])
			) {
				set_varmap(await set_variable(split[0], split[1], var_map));
				context.serialized_function = "";
				if (split[1].endsWith("i")) {
					return [split[0], parseFloat(split[1].slice(0, -1)), true];
				} else {
					return [split[0], parseFloat(split[1]), false];
				}
			} else {
				console.error("Invalid variable assignment");
			}
		} else {
			context.serialized_function = await parse_string(value);
		}
		return ["", 0, false];
	}
}

export class function_text_inputs {
	public function_boxes: Array<function_box>;
	public container: HTMLDivElement;
	public context_map: Map<number, proceduralOffscreen>;
	constructor(map: Map<number, proceduralOffscreen>) {
		this.function_boxes = [];
		this.container = <HTMLDivElement>document.getElementById("inputs");
		this.context_map = map;
		let first_box = new function_box(this, map);
		this.function_boxes.push(first_box);
		this.container.appendChild(this.function_boxes[0].container);
	}
}

export class metaUIContainer {
	public horizontal_axis_selector: HTMLSelectElement;
	public vertical_axis_selector: HTMLSelectElement;
	public horizontal_axis: string;
	public vertical_axis: string;
	public reset_view_button: HTMLButtonElement;
	public frame_time: HTMLParagraphElement;
	public continuity_toggle: HTMLInputElement;
	public continuity: boolean;
	public bounds_inputs: Array<HTMLInputElement>;
	public resolution_input: HTMLInputElement;
	public resolution_up_stepper: HTMLButtonElement;
	public resolution_down_stepper: HTMLButtonElement;
	public resolution: number;
	public slice_slider: HTMLInputElement;
	public slice: number;
	public slice_min_input: HTMLInputElement;
	public slice_max_input: HTMLInputElement;
	public slice_input: HTMLInputElement;

	constructor() {
		// User to select which axes are rendered
		this.horizontal_axis_selector = <HTMLSelectElement>(
			document.getElementById("haxis")
		);
		this.vertical_axis_selector = <HTMLSelectElement>(
			document.getElementById("vaxis")
		);
		this.horizontal_axis = "i_r";
		this.vertical_axis = "o_r";
		this.reset_view_button = <HTMLButtonElement>(
			document.getElementById("resetView")
		);
		this.frame_time = <HTMLParagraphElement>(
			document.getElementById("frameTime")
		);
		this.continuity_toggle = <HTMLInputElement>document.getElementById("cont");
		this.continuity = true;
		this.bounds_inputs = [
			<HTMLInputElement>document.getElementById("x1"),
			<HTMLInputElement>document.getElementById("x2"),
			<HTMLInputElement>document.getElementById("y1"),
			<HTMLInputElement>document.getElementById("y2"),
		];
		this.resolution_input = <HTMLInputElement>(
			document.getElementById("resolution")
		);
		this.resolution_up_stepper = <HTMLButtonElement>(
			document.getElementById("rezUp")
		);
		this.resolution_down_stepper = <HTMLButtonElement>(
			document.getElementById("rezDown")
		);
		this.resolution = 3;
		this.slice_slider = <HTMLInputElement>document.getElementById("slice");
		this.slice = 0;
		this.slice_min_input = <HTMLInputElement>(
			document.getElementById("minSlice")
		);
		this.slice_max_input = <HTMLInputElement>(
			document.getElementById("maxSlice")
		);
		this.slice_input = <HTMLInputElement>document.getElementById("sliceVal");
		init_slice(this);
		init_axes(this);
		init_event_listeners(this);
	}

	set_bounds_inputs(index: number, value: number) {
		if (this.bounds_inputs[index]) {
			this.bounds_inputs[index].value = String(value);
		}
	}
}

function init_slice(metaUIobj: metaUIContainer) {
	function update_slice_range() {
		if (
			metaUIobj.slice_slider &&
			metaUIobj.slice_min_input &&
			metaUIobj.slice_max_input
		) {
			if (metaUIobj.slice_min_input && metaUIobj.slice_max_input) {
				metaUIobj.slice_slider.setAttribute(
					"min",
					String(parseFloat(metaUIobj.slice_min_input.value)),
				);

				metaUIobj.slice_slider.setAttribute(
					"max",
					String(parseFloat(metaUIobj.slice_max_input.value)),
				);
				metaUIobj.slice_slider.setAttribute(
					"step",
					String(
						(parseFloat(metaUIobj.slice_max_input.value) -
							parseFloat(metaUIobj.slice_min_input.value) / 2) /
						200,
					),
				);
			}
		}
	}

	(<any>window).updateRange = update_slice_range;

	if (
		metaUIobj.slice_slider &&
		metaUIobj.slice_min_input &&
		metaUIobj.slice_max_input
	) {
		metaUIobj.slice_slider.oninput = async function() {
			if (
				metaUIobj.slice_slider &&
				metaUIobj.slice_min_input &&
				metaUIobj.slice_max_input
			) {
				let val: HTMLInputElement | null = <HTMLInputElement>(
					document.getElementById("sliceVal")
				);
				if (val) {
					val.value = metaUIobj.slice_slider.value;
					metaUIobj.slice = parseFloat(metaUIobj.slice_slider.value);
					await render();
				}
			}
		};

		if (metaUIobj.slice_input) {
			metaUIobj.slice_input.oninput = async function() {
				if (metaUIobj.slice_input && metaUIobj.slice_slider) {
					metaUIobj.slice_slider.value = String(
						parseFloat(metaUIobj.slice_input.value),
					);
					metaUIobj.slice = parseFloat(metaUIobj.slice_input.value);
					await render();
				}
			};
		}
	}
}

function init_axes(metaUIobj: metaUIContainer) {
	async function on_axis_change() {
		if (
			metaUIobj.horizontal_axis_selector &&
			metaUIobj.vertical_axis_selector
		) {
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

function init_event_listeners(metaUIobj: metaUIContainer) {
	if (metaUIobj.reset_view_button) {
		metaUIobj.reset_view_button.addEventListener("click", async function() {
			graph.reset(mainCanvasContext);
			graph.updateBounds();
			await render();
		});
	}

	if (metaUIobj.continuity_toggle) {
		metaUIobj.continuity_toggle.addEventListener("click", async function() {
			metaUIobj.continuity = !metaUIobj.continuity;
			await render();
		});
	}

	if (metaUIobj.bounds_inputs) {
		for (let [index, val] of metaUIobj.bounds_inputs.entries()) {
			if (val) {
				val.oninput = async function() {
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
		metaUIobj.resolution_input.addEventListener("change", async function() {
			metaUIobj.resolution = parseInt(metaUIobj.resolution_input.value);
			await render();
		});
		if (metaUIobj.resolution_up_stepper) {
			metaUIobj.resolution_up_stepper.addEventListener(
				"click",
				async function() {
					metaUIobj.resolution = parseInt(metaUIobj.resolution_input.value);
					await render();
				},
			);
		}
		if (metaUIobj.resolution_down_stepper) {
			metaUIobj.resolution_down_stepper.addEventListener(
				"click",
				async function() {
					metaUIobj.resolution = parseInt(metaUIobj.resolution_input.value);
					await render();
				},
			);
		}
	}

	if (metaUIobj.slice_slider) {
		metaUIobj.slice_slider.addEventListener("change", async function() {
			metaUIobj.slice = parseFloat(metaUIobj.slice_slider.value);
			await render();
		});
	}
}
