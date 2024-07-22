import * as wasm from "./index.js";
//document.addEventListener('DOMContentLoaded', (event) => {
//wasm.ensureWasmInit().then(() => {
const sizer = document.getElementById("scalar");
const resetView = document.getElementById("resetView");
const ft = document.getElementById("frameTime");

let haxis = document.getElementById("haxis");
let vaxis = document.getElementById("vaxis");
let horizontal = "i_r"
let vertical = "o_r"
async function onChange() {
  horizontal = haxis.value;
  vertical = vaxis.value;
  render();
}
haxis.onchange = onChange;
vaxis.onchange = onChange;

let continuity = document.getElementById("cont");
let contCheck = true;
continuity.addEventListener("click", function(){contCheck = !contCheck; render()});


// Creating a canvas object to contain the element itsself and
// the info used surrounding
let canvas = {
  object: document.getElementById("glcanvas"),
  init: function () {
    this.style = getComputedStyle(canvas.object);
    this.width = parseFloat(this.style.getPropertyValue("width"));
    this.height = parseFloat(this.style.getPropertyValue("height"));
    this.aspectRatio = canvas.height / canvas.width;
  },
  resetBitmap: function () {
    canvas.object.style.height = canvas.height + "px";
    canvas.object.style.width = canvas.width + "px";
    canvas.object.height = canvas.height;
    canvas.object.width = canvas.width;
  }
}
canvas.init();


// Initialize the GL context
const cnv = canvas.object.getContext("2d");
canvas.resetBitmap();
cnv.font = "15px serif";
cnv.strokeStyle = "white";
cnv.lineWidth = 4;
cnv.textAlign = "center";
let click = false;

// Graph object contains all info needed to draw itsself to screen

let graph = {
  initialBounds: [-10, 10, -10 * canvas.aspectRatio, 10 * canvas.aspectRatio], // -x,x,-y,y
  zoom: 0,
  screenTarget: [0, 0],
  zoomLog: 1,
  sensitivity: 0.0217791 * canvas.aspectRatio,
  init: function () {
    this.bounds = JSON.parse(JSON.stringify(graph.initialBounds));
    this.width = graph.bounds[1] - graph.bounds[0],
      this.height = graph.bounds[3] - graph.bounds[2],
      this.scaleFactor = [
        Math.pow(10, Math.floor(Math.log10(graph.width))),
        Math.pow(10, Math.floor(Math.log10(graph.height))),
      ];
  },
  reset: function () {
    this.zoom = 0;
    this.screenTarget = [0, 0];
    this.zoomLog = 1;
  },
  updateBounds: function () {
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
  },
  toScreenspace: function (real, imag) {
    let normReal = 1 - ((this.bounds[1] - real) / (this.bounds[1] - this.bounds[0]));
    let normImag = (this.bounds[3] - imag) / (this.bounds[3] - this.bounds[2]);

    return [normReal * canvas.width, normImag * canvas.height];

  },
  unScreenspace: function (real, imag) {
    let normR = real / canvas.width;
    let normI = imag / canvas.height;

    normR = this.bounds[1] - ((1 - normR) * (this.bounds[1] - this.bounds[0]))
    normI = this.bounds[3] - (imag * (this.bounds[3] - this.bounds[2]));
    return [normR, normI];

  }
};

graph.init();
render();

async function resize() {
  canvas.resetBitmap();
  canvas.init();
  cnv.font = "15px serif";
  cnv.strokeStyle = "white";
  cnv.textAlign = "center";
  graph.initialBounds[2] = graph.initialBounds[0] * canvas.aspectRatio;
  graph.initialBounds[3] = graph.initialBounds[1] * canvas.aspectRatio;
  graph.sensitivity = 0.0217792 * canvas.aspectRatio;
  graph.updateBounds();
  await render();
};

CanvasRenderingContext2D.prototype.curve = function (h, r, f, c) { r = (typeof r === "number") ? r : 0.5; f = f ? f : 20; var j, k = [], e = h.length, d, a = new Float32Array((f + 2) * 4), b = 4; j = h.slice(0); if (c) { j.unshift(h[e - 1]); j.unshift(h[e - 2]); j.push(h[0], h[1]) } else { j.unshift(h[1]); j.unshift(h[0]); j.push(h[e - 2], h[e - 1]) } a[0] = 1; for (d = 1; d < f; d++) { var m = d / f, n = m * m, p = n * m, o = p * 2, q = n * 3; a[b++] = o - q + 1; a[b++] = q - o; a[b++] = p - 2 * n + m; a[b++] = p - n } a[++b] = 1; g(j, a, e); if (c) { j = []; j.push(h[e - 4], h[e - 3], h[e - 2], h[e - 1]); j.push(h[0], h[1], h[2], h[3]); g(j, a, 4) } function g(B, u, w) { for (var v = 2; v < w; v += 2) { var x = B[v], y = B[v + 1], z = B[v + 2], A = B[v + 3], D = (z - B[v - 2]) * r, E = (A - B[v - 1]) * r, F = (B[v + 4] - x) * r, G = (B[v + 5] - y) * r; for (var C = 0; C <= f; C++) { var s = C * 4; k.push(u[s] * x + u[s + 1] * z + u[s + 2] * D + u[s + 3] * F, u[s] * y + u[s + 1] * A + u[s + 2] * E + u[s + 3] * G) } } } for (d = 0, e = k.length; d < e; d += 2) { this.lineTo(k[d], k[d + 1]) } return k };

export async function render() {
  let frameTime = new Date;
  cnv.clearRect(0, 0, canvas.width, canvas.height);
  let values = await wasm.squaredvals(graph.bounds, canvas.width, canvas.height, horizontal,vertical);

  //drawCurve(cnv,pts);
  let points = [];
  for (let i = 0; i < Math.sqrt(values.length * 1.8); i += 2) {
    points.push(values[i], values[i + 1]);
    if (!contCheck){
    cnv.fillRect(points[i] - 1, points[i + 1] - 1, 3, 3);
    }
  }
  if (contCheck) {
  console.log(points);
  cnv.strokeStyle = "black";
  cnv.beginPath();
  //cnv.moveTo(points[0], points[1]);
  cnv.curve(points);
  //cnv.lineTo(points.at(-2), points.at(-1));
  cnv.stroke()
  cnv.strokeStyle = "white";
  }
  ft.innerText = ("Render time: " + (new Date - frameTime) + "ms\n Dots to render:" + Math.round(Math.sqrt(values.length * 1.8)) + "\nZoom: " + 0.5 / graph.zoomLog + "\nScale Factor x: " + graph.scaleFactor[0] + "\nScale Factor y: " + graph.scaleFactor[1] + "\nCanvas width: " + canvas.width + "\nCanvas height: " + canvas.height + "\nBounds: " + graph.bounds.map(val => Math.round(1000 * val) / 1000));
  grid();
}

function gridline(int) {
  let opts = [100, 50, 20, 10, 5, 2, 1, 0.5, 0.2, 0.1, 0.05];
  for (let j = 0; j < opts.length - 1; j++) {
    if (opts[j] * graph.scaleFactor[int] < 4 / graph.zoomLog) {
      return opts[j] * graph.scaleFactor[int];
    }
  }
}

function superFloor(mult, val) {
  return mult * Math.floor(val / mult);
}

function precision(a) {
  if (!isFinite(a)) return 0;
  var e = 1, p = 0;
  while (Math.round(a * e) / e !== a) { e *= 10; p++; }
  return p;
}

function grid() {
  // Origin lines
  let origin = graph.toScreenspace(0, 0);
  cnv.fillRect(origin[0] - 1, 0, 2, canvas.height);
  cnv.fillRect(0, origin[1] - 1, canvas.width, 2);
  let xScale = gridline(0);
  let yScale = gridline(1);

  // X axis is always fixed with the aspect ratio, Y is variable
  // so we make a different amount of lines for each to eliminiate draw calls

  let text;

  //Major X lines
  for (let i = -7; i < 8; i++) {
    let xpos = graph.toScreenspace(
      xScale * i + superFloor(xScale, graph.screenTarget[0]),
      0,
    );


    cnv.fillRect(xpos[0], 0, 0.5, canvas.height);
    // Major x text
    text = xScale * i + superFloor(xScale, graph.screenTarget[0]);
    text = precision(text) == 0 ? Math.round(text) : text;
    cnv.strokeText(
      text,
      xpos[0],
      Math.min(Math.max(xpos[1] + 18, 14), canvas.height - 8),
      150,
    );
    cnv.fillText(
      text,
      xpos[0],
      Math.min(Math.max(xpos[1] + 18, 14), canvas.height - 8),
      150,
    );

  }
  //Minor X lines
  for (let i = -25; i < 25; i++) {
    cnv.fillRect(
      graph.toScreenspace(
        (xScale / 5) * i + superFloor(xScale / 5, graph.screenTarget[0]),
        0,
      )[0],
      0,
      0.1,
      canvas.height,
    );
  }
  //Major Y Lines
  for (let i = -7 * Math.ceil(canvas.aspectRatio); i < 8 * Math.ceil(canvas.aspectRatio); i++) {
    let ypos = graph.toScreenspace(
      0,
      yScale * i + superFloor(yScale, graph.screenTarget[1]),
    );

    cnv.fillRect(0, ypos[1], canvas.width, 0.5);
    //major Y text";
    text = yScale * i + superFloor(yScale, graph.screenTarget[1]);
    text = precision(text) == 0 ? Math.round(text) : text;
    if (ypos[0] - 15 < 10) {
      cnv.textAlign = "left";
    } else if (ypos[0] - 15 > canvas.width - 10) {
      cnv.textAlign = "right";
    }
    cnv.strokeText(
      text,
      Math.min(Math.max(ypos[0] - 15, 10), canvas.width - 10),
      ypos[1] + 4,
      150,
    );
    cnv.fillText(
      text,
      Math.min(Math.max(ypos[0] - 15, 10), canvas.width - 10),
      ypos[1] + 4,
      150,
    );
    cnv.textAlign = "center";
  }
  //Minor Y lines
  for (let i = -30 * Math.ceil(canvas.aspectRatio); i < 30 * Math.ceil(canvas.aspectRatio); i++) {
    cnv.fillRect(
      0,
      graph.toScreenspace(
        0,
        (yScale / 5) * i + superFloor(yScale / 5, graph.screenTarget[1]),
      )[1],
      canvas.width,
      0.1,
    );
  }

}

let initialScreenTarget = [0, 0];
let mouseStart = [0, 0];

canvas.object.addEventListener("mousedown", function (e) {
  mouseStart = [e.clientX, e.clientY]
  initialScreenTarget[0] = graph.screenTarget[0];
  initialScreenTarget[1] = graph.screenTarget[1];
  click = true;
});
addEventListener("mouseup", function (e) {
  click = false;
});

addEventListener("wheel", async function (event) {
  graph.zoom += event.deltaY * -0.001;
  graph.zoomLog = Math.pow(2, graph.zoom);
  graph.updateBounds();
  await render();
  console.log("zoomed: " + event.deltaY);
}, { passive: false });

let prevCoords = []

canvas.object.addEventListener("mousemove", async function (e) {
  if (click) {
    graph.screenTarget[0] = parseFloat(initialScreenTarget[0] - graph.sensitivity * (e.clientX - mouseStart[0]) / graph.zoomLog);
    graph.screenTarget[1] = parseFloat(initialScreenTarget[1] + graph.sensitivity * (e.clientY - mouseStart[1]) / graph.zoomLog);
    graph.updateBounds();
    await render();
  }
  if ([e.clientX, e.clientY] != prevCoords) {
    prevCoords = [e.clientX, e.clientY];
  }
});

let sizeMove = false;
sizer.addEventListener("mousedown", function (e) {
  canvas.init();
  sizeMove = true;
});
let inpBloc = document.getElementById("input");
window.addEventListener("mousemove", function (e) {
  let current_margin = parseFloat(getComputedStyle(inpBloc).getPropertyValue("margin-right"));
  if (sizeMove && (current_margin >= 0 || e.movementX > 0)) {
    canvas.width -= e.movementX;

    inpBloc.style.marginRight = current_margin + e.movementX + "px";
    resize();
  }
});

window.addEventListener("mouseup", function (e) {
  sizeMove = false;
});

resetView.addEventListener("click", async function () {
  graph.reset();
  graph.updateBounds();
  await render();
  grid();
});
//});
//});