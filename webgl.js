const canvas = document.getElementById("glcanvas");
const resetView = document.getElementById("resetView");
const x1 = document.getElementById("x1");
const x2 = document.getElementById("x2");
const y1 = document.getElementById("y1");
const y2 = document.getElementById("y2");
// Initialize the GL context
const cnv = canvas.getContext("2d");
cnv.font = "12px serif";
cnv.strokeStyle = "white";
cnv.lineWidth = 4;
cnv.textAlign = "center";
let canvasWidth = canvas.width;
let canvasHeight = canvas.height;
let aspectRatio = canvasWidth / canvasHeight;
let graphBounds = [-10, 10, -10, 10]; // -x,x,-y,y
let graphInfo = {
  width: graphBounds[1] - graphBounds[0],
  height: graphBounds[3] - graphBounds[2],
  zoom: 0,
};
graphInfo.scaleFactor = [
  Math.pow(10, Math.floor(Math.log10(graphInfo.width))),
  Math.pow(10, Math.floor(Math.log10(graphInfo.height))),
];
let screenTarget = [0, 0];
let sens = 24;
let zoomLog = 1;
let click = false;

function toScreenspace(real, imag) {
  let normReal =
    1 - (graphBounds[1] - real) / (graphBounds[1] - graphBounds[0]);
  let normImag = (graphBounds[3] - imag) / (graphBounds[3] - graphBounds[2]);
  return [normReal * canvasWidth, normImag * canvasHeight];
}

resetView.addEventListener("click", function () {
  graphInfo.zoom = 0;
  graphInfo.zoom = 0;
  zoomLog = 1;
  screenTarget = [0, 0];
  graphBounds = [-10, 10, -10, 10];
  updateBounds();
  drawPoint();
  grid();
});

function drawPoint() {
  cnv.clearRect(0, 0, canvasWidth, canvasHeight);
  let origin = toScreenspace(0, 0);
  cnv.fillRect(origin[0] - 1, 0, 2, canvasHeight);
  cnv.fillRect(0, origin[1] - 1, canvasWidth, 2);
  grid();
}

function gridline(int) {
  let opts = [100, 50, 20, 10, 5, 2, 1, 0.5, 0.2, 0.1, 0.05];
  for (let j = 0; j < opts.length - 1; j++) {
    if (opts[j] * graphInfo.scaleFactor[int] < 10 / zoomLog) {
      return opts[j] * graphInfo.scaleFactor[int];
    }
  }
}

function superFloor(mult, val) {
  return mult * Math.floor(val / mult);
}
function grid() {
  let xScale = gridline(0);
  let yScale = gridline(1);

  for (let i = -10; i < 10; i++) {
    cnv.fillRect(
      toScreenspace(
        (xScale / 5) * i + superFloor(xScale / 5, screenTarget[0]),
        0,
      )[0],
      0,
      0.1,
      canvasHeight,
    );
    cnv.fillRect(
      0,
      toScreenspace(
        0,
        (yScale / 5) * i + superFloor(yScale / 5, screenTarget[1]),
      )[1],
      canvasWidth,
      0.1,
    );
  }

  for (let i = -2; i < 3; i++) {
    let xpos = toScreenspace(
      xScale * i + superFloor(xScale, screenTarget[0]),
      0,
    );
    let ypos = toScreenspace(
      0,
      yScale * i + superFloor(yScale, screenTarget[1]),
    );
    //Major X lines
    cnv.fillRect(xpos[0], 0, 0.5, canvasHeight);
    // Major x text
    let text = xScale * i + superFloor(xScale, screenTarget[0]);
    text = text >= 1 ? Math.round(text) : text;
    cnv.strokeText(
      text,
      xpos[0],
      Math.min(Math.max(xpos[1] + 18, 14), canvasHeight - 8),
      100,
    );
    cnv.fillText(
      text,
      xpos[0],
      Math.min(Math.max(xpos[1] + 18, 14), canvasHeight - 8),
      100,
    );
    //Major Y Lines
    cnv.fillRect(0, ypos[1], canvasWidth, 0.5);
    //major Y text";
    text = yScale * i + superFloor(yScale, screenTarget[1]);
    text = text >= 1 ? Math.round(text) : text;
    cnv.strokeText(
      text,
      Math.min(Math.max(ypos[0] - 15, 10), canvasWidth - 20),
      ypos[1] + 4,
      100,
    );
    cnv.fillText(
      text,
      Math.min(Math.max(ypos[0] - 15, 10), canvasWidth - 20),
      ypos[1] + 4,
      100,
    );
  }

  console.log(gridline());
}

function updateBounds() {
  let inverseZL = 1 / zoomLog;
  graphBounds[0] = screenTarget[0] - 10 * aspectRatio * inverseZL;
  graphBounds[1] = screenTarget[0] + 10 * aspectRatio * inverseZL;
  graphBounds[2] = screenTarget[1] - 10 * inverseZL;
  graphBounds[3] = screenTarget[1] + 10 * inverseZL;
  graphInfo.width = graphBounds[1] - graphBounds[0];
  graphInfo.height = graphBounds[3] - graphBounds[2];
  graphInfo.scaleFactor = [
    Math.pow(10, Math.floor(Math.log10(graphInfo.width))),
    Math.pow(10, Math.floor(Math.log10(graphInfo.height))),
  ];
}

drawPoint();
canvas.addEventListener("mousedown", function (e) {
  click = true;
});
addEventListener("mouseup", function (e) {
  click = false;
});
addEventListener("wheel", (event) => {
  graphInfo.zoom += event.deltaY * -0.0008;
  zoomLog = Math.pow(10, graphInfo.zoom);
  updateBounds();

  drawPoint();
});
canvas.addEventListener("mousemove", function (e) {
  if (click) {
    screenTarget[0] = screenTarget[0] - e.movementX * (1 / (zoomLog * sens));
    screenTarget[1] = screenTarget[1] + e.movementY * (1 / (zoomLog * sens));
    updateBounds();
    drawPoint();
  }
});
