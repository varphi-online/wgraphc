const sizer = document.getElementById("scalar");
const resetView = document.getElementById("resetView");

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
    let normReal =
      1 - (this.bounds[1] - real) / (this.bounds[1] - this.bounds[0]);
    let normImag = (this.bounds[3] - imag) / (this.bounds[3] - this.bounds[2]);
    return [normReal * canvas.width, normImag * canvas.height];

  }
};

graph.init();
render();

function resize() {
  canvas.resetBitmap();
  canvas.init();
  cnv.font = "15px serif";
  cnv.strokeStyle = "white";
  cnv.textAlign = "center";
  graph.initialBounds[2] = graph.initialBounds[0] * canvas.aspectRatio;
  graph.initialBounds[3] = graph.initialBounds[1] * canvas.aspectRatio;
  graph.sensitivity = 0.0217792 * canvas.aspectRatio;
  graph.updateBounds();
  render();
};

function render() {
  cnv.clearRect(0, 0, canvas.width, canvas.height);
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

addEventListener("wheel", (event) => {
  graph.zoom += event.deltaY * -0.001;
  graph.zoomLog = Math.pow(2, graph.zoom);
  graph.updateBounds();

  render();
});

let prevCoords = []

canvas.object.addEventListener("mousemove", function (e) {
  if (click) {
    graph.screenTarget[0] = parseFloat(initialScreenTarget[0] - graph.sensitivity * (e.clientX - mouseStart[0]) / graph.zoomLog);
    graph.screenTarget[1] = parseFloat(initialScreenTarget[1] + graph.sensitivity * (e.clientY - mouseStart[1]) / graph.zoomLog);
    graph.updateBounds();
    render();
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

resetView.addEventListener("click", function () {
  graph.reset();
  graph.updateBounds();
  render();
  grid();
});