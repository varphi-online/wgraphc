import { render, debug_mode } from "./index.js";
import { metaUIContainer } from "./metaUI.js";
import { oscDraw } from "./wasm.js";
export let metaUI = new metaUIContainer();
// Creating a canvas object to contain the element itsself and
// the info used surrounding
export class canvasContainer {
    style;
    object;
    width;
    height;
    aspectRatio;
    initialScreenTarget;
    mouseStart;
    click;
    constructor() {
        this.object = document.getElementById("canvas");
        this.initialScreenTarget = [0, 0];
        this.mouseStart = [0, 0];
        this.click = false;
    }
    init() {
        this.style = getComputedStyle(this.object);
        this.width = parseFloat(this.style.getPropertyValue("width"));
        this.height = parseFloat(this.style.getPropertyValue("height"));
        this.aspectRatio = this.height / this.width;
    }
    event_init(self, graph) {
        this.object.addEventListener("mousedown", function (e) {
            self.mouseStart = [e.clientX, e.clientY];
            self.initialScreenTarget[0] = graph.screenTarget[0];
            self.initialScreenTarget[1] = graph.screenTarget[1];
            self.click = true;
        });
        this.object.addEventListener("wheel", async function (event) {
            graph.zoom += event.deltaY * -0.001;
            graph.zoomLog = Math.pow(2, graph.zoom);
            graph.updateBounds();
            await render();
        }, { passive: false });
        this.object.addEventListener("mousemove", async function (e) {
            if (self.click) {
                graph.screenTarget[0] =
                    self.initialScreenTarget[0] -
                        (graph.sensitivity * (e.clientX - self.mouseStart[0])) /
                            graph.zoomLog;
                graph.screenTarget[1] =
                    self.initialScreenTarget[1] +
                        (graph.sensitivity * (e.clientY - self.mouseStart[1])) /
                            graph.zoomLog;
                graph.updateBounds();
                await render();
            }
        });
        window.addEventListener("mouseup", function () {
            self.click = false;
        });
    }
    resetBitmap(context) {
        this.object.style.height = this.height + "px";
        this.object.style.width = this.width + "px";
        this.object.height = this.height;
        this.object.width = this.width;
        this.aspectRatio = this.height / this.width;
        context.font = "15px serif";
        context.strokeStyle = "white";
        context.lineWidth = 3;
        context.textAlign = "center";
    }
}
// Graph object contains all info needed to draw itsself to screen
export class graphContainer {
    initialBounds;
    zoom;
    zoomLog;
    sensitivity;
    screenTarget;
    bounds;
    width;
    height;
    scaleFactor;
    constructor(canvas) {
        this.initialBounds = [
            -10,
            10,
            -10 * canvas.aspectRatio,
            10 * canvas.aspectRatio,
        ]; // -x,x,-y,y
        this.zoom = 0;
        this.screenTarget = [0, 0];
        this.zoomLog = 1;
        this.sensitivity = 0.0217791 * canvas.aspectRatio;
    }
    init() {
        this.bounds = JSON.parse(JSON.stringify(this.initialBounds));
        (this.width = this.bounds[1] - this.bounds[0]),
            (this.height = this.bounds[3] - this.bounds[2]),
            (this.scaleFactor = [
                Math.pow(10, Math.floor(Math.log10(this.width))),
                Math.pow(10, Math.floor(Math.log10(this.height))),
            ]);
        metaUI.set_bounds_inputs(0, this.bounds[0]);
        metaUI.set_bounds_inputs(1, this.bounds[1]);
        metaUI.set_bounds_inputs(2, this.bounds[2]);
        metaUI.set_bounds_inputs(3, this.bounds[3]);
    }
    reset(context) {
        this.zoom = 0;
        this.screenTarget = [0, 0];
        this.zoomLog = 1;
        context.font = "15px serif";
        context.strokeStyle = "white";
        context.lineWidth = 3;
        context.textAlign = "center";
    }
    updateBounds() {
        let inverseZL = 1 / this.zoomLog;
        this.bounds[0] = this.screenTarget[0] + this.initialBounds[0] * inverseZL;
        this.bounds[1] = this.screenTarget[0] + this.initialBounds[1] * inverseZL;
        this.bounds[2] = this.screenTarget[1] + this.initialBounds[2] * inverseZL;
        this.bounds[3] = this.screenTarget[1] + this.initialBounds[3] * inverseZL;
        this.width = this.bounds[1] - this.bounds[0];
        this.height = this.bounds[3] - this.bounds[2];
        this.scaleFactor = [
            Math.pow(10, Math.floor(Math.log10(this.width))),
            Math.pow(10, Math.floor(Math.log10(this.height))),
        ];
        metaUI.set_bounds_inputs(0, this.bounds[0]);
        metaUI.set_bounds_inputs(1, this.bounds[1]);
        metaUI.set_bounds_inputs(2, this.bounds[2]);
        metaUI.set_bounds_inputs(3, this.bounds[3]);
    }
    toScreenspace(real, imag, canvas) {
        let normReal = 1 - (this.bounds[1] - real) / (this.bounds[1] - this.bounds[0]);
        let normImag = (this.bounds[3] - imag) / (this.bounds[3] - this.bounds[2]);
        return [normReal * canvas.width, normImag * canvas.height];
    }
    resize(canvas) {
        this.initialBounds[2] = this.initialBounds[0] * canvas.aspectRatio;
        this.initialBounds[3] = this.initialBounds[1] * canvas.aspectRatio;
        this.updateBounds();
        this.sensitivity = 0.0217791 * canvas.aspectRatio;
        render();
    }
    async render(canvas, ctx, graph, offscreens, vars) {
        let frameTime = new Date().getTime();
        ctx.fillStyle = "white";
        ctx.fillRect(0, 0, canvas.width, canvas.height);
        ctx.fillStyle = "black";
        this.grid(canvas, ctx, graph);
        if (offscreens) {
            for (let i = 0; i < offscreens.length; i++) {
                await oscDraw(offscreens[i], canvas, graph, metaUI, vars);
                ctx.drawImage(offscreens[i].object, 0, 0, canvas.width, canvas.height);
            }
        }
        if (metaUI.frame_time) {
            let debug = debug_mode ? " (Debug)" : "";
            metaUI.frame_time.innerText =
                String(new Date().getTime() - frameTime) + "ms" + debug;
        }
    }
    //BUG: Fix for user-defined aspect ratios because grid lines are not drawn
    //properly at extremes
    gridline(int, graph) {
        let opts = [100, 50, 20, 10, 5, 2, 1, 0.5, 0.2, 0.1, 0.05];
        for (let j = 0; j < opts.length - 1; j++) {
            if (opts[j] * graph.scaleFactor[int] < 4 / graph.zoomLog) {
                return opts[j] * graph.scaleFactor[int];
            }
        }
    }
    superFloor(mult, val) {
        return mult * Math.floor(val / mult);
    }
    precision(a) {
        if (!isFinite(a))
            return 0;
        var e = 1, p = 0;
        while (Math.round(a * e) / e !== a) {
            e *= 10;
            p++;
        }
        return p;
    }
    grid(canvas, ctx, graph) {
        // Origin lines
        let origin = graph.toScreenspace(0, 0, canvas);
        ctx.fillRect(origin[0] - 1, 0, 2, canvas.height);
        ctx.fillRect(0, origin[1] - 1, canvas.width, 2);
        let xScale = this.gridline(0, graph);
        let yScale = this.gridline(1, graph);
        // X axis is always fixed with the aspect ratio, Y is variable
        // so we make a different amount of lines for each to eliminiate draw calls
        let text;
        let xAdd = metaUI.horizontal_axis.endsWith("i") ? "i" : "";
        let yAdd = metaUI.vertical_axis.endsWith("i") ? "i" : "";
        //Major X lines
        for (let i = -7; i < 8; i++) {
            let xpos = graph.toScreenspace(xScale * i + this.superFloor(xScale, graph.screenTarget[0]), 0, canvas);
            ctx.fillRect(xpos[0], 0, 0.5, canvas.height);
            // Major x text
            text = xScale * i + this.superFloor(xScale, graph.screenTarget[0]);
            text = this.precision(text) == 0 ? Math.round(text) : text;
            ctx.strokeText(text + xAdd, xpos[0], Math.min(Math.max(xpos[1] + 18, 14), canvas.height - 8), 150);
            ctx.fillText(text + xAdd, xpos[0], Math.min(Math.max(xpos[1] + 18, 14), canvas.height - 8), 150);
        }
        //Minor X lines
        for (let i = -25; i < 25; i++) {
            ctx.fillRect(graph.toScreenspace((xScale / 5) * i + this.superFloor(xScale / 5, graph.screenTarget[0]), 0, canvas)[0], 0, 0.1, canvas.height);
        }
        //Major Y Lines
        for (let i = -7 * Math.ceil(canvas.aspectRatio); i < 8 * Math.ceil(canvas.aspectRatio); i++) {
            let ypos = graph.toScreenspace(0, yScale * i + this.superFloor(yScale, graph.screenTarget[1]), canvas);
            ctx.fillRect(0, ypos[1], canvas.width, 0.5);
            //major Y text";
            text = yScale * i + this.superFloor(yScale, graph.screenTarget[1]);
            text = this.precision(text) == 0 ? Math.round(text) : text;
            if (ypos[0] - 15 < 10) {
                ctx.textAlign = "left";
            }
            else if (ypos[0] - 15 > canvas.width - 10) {
                ctx.textAlign = "right";
            }
            ctx.strokeText(text + yAdd, Math.min(Math.max(ypos[0] - 15, 10), canvas.width - 10), ypos[1] + 4, 150);
            ctx.fillText(text + yAdd, Math.min(Math.max(ypos[0] - 15, 10), canvas.width - 10), ypos[1] + 4, 150);
            ctx.textAlign = "center";
        }
        //Minor Y lines
        for (let i = -30 * Math.ceil(canvas.aspectRatio); i < 30 * Math.ceil(canvas.aspectRatio); i++) {
            ctx.fillRect(0, graph.toScreenspace(0, (yScale / 5) * i + this.superFloor(yScale / 5, graph.screenTarget[1]), canvas)[1], canvas.width, 0.1);
        }
    }
}
export class proceduralOffscreen {
    object;
    width;
    height;
    context;
    serialized_function;
    color;
    draw;
    constructor() {
        this.object = new OffscreenCanvas(window.innerWidth, window.innerHeight);
        this.width = window.innerWidth;
        this.height = window.innerHeight;
        this.context = (this.object.getContext("2d"));
        this.context.strokeStyle = "black";
        this.serialized_function = "";
        this.resetBitmap();
        this.color = ["red", "green", "blue", "orange"][Math.floor(Math.random() * 4)];
    }
    resetBitmap() {
        this.object.height = this.height;
        this.object.width = this.width;
    }
    set_draw(value) {
        this.draw = value;
    }
}
