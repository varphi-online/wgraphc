const canvas = document.getElementById("glcanvas");
// Initialize the GL context
const cnv = canvas.getContext("2d");
let canvasWidth = canvas.width;
let canvasHeight = canvas.height;
let canvasBounds = [-10, 10, -10, 10]; // -x,x,-y,y
let mouseDelta = [];
let originOffset = [0, 0];
let sens = 10;
let zoom = 0;
let zoomLog = 1;
let click = false;

function toScreenspace(real, imag) {
  let normReal =
    1 - (canvasBounds[1] - real) / (canvasBounds[1] - canvasBounds[0]);
  let normImag = (canvasBounds[3] - imag) / (canvasBounds[3] - canvasBounds[2]);
  console.log([normReal, normImag]);
  return [normReal * canvasWidth, normImag * canvasHeight];
}

console.log(canvasBounds);
console.log(toScreenspace(5, 0));

function drawPoint() {
  cnv.clearRect(0, 0, canvas.width, canvas.height);
  cnv.fillRect(canvasWidth / 2 + originOffset[0], originOffset[1], 2, 99999);
  cnv.fillRect(originOffset[0], canvasHeight / 2 + originOffset[1], 99999, 2);
  cnv.fillRect(...toScreenspace(5, 1), 2, 2);
  cnv.fillRect(...toScreenspace(-2, 7), 2, 2);
  cnv.fillRect(...toScreenspace(-3, -3), 2, 2);
}
function updateBounds() {
  canvasBounds[0] = originOffset[0] - 10 * (1 / zoomLog);
  canvasBounds[1] = originOffset[0] + 10 * (1 / zoomLog);
  canvasBounds[2] = originOffset[1] - 10 * (1 / zoomLog);
  canvasBounds[3] = originOffset[1] + 10 * (1 / zoomLog);
  console.log(canvasBounds);
}
drawPoint();
canvas.addEventListener("mousedown", function (e) {
  click = true;
});
addEventListener("mouseup", function (e) {
  click = false;
});
addEventListener("wheel", (event) => {
  zoom += event.deltaY * -0.0001;
  zoomLog = Math.pow(10, zoom);
  updateBounds();

  drawPoint();
});
canvas.addEventListener("mousemove", function (e) {
  if (click) {
    originOffset[0] = originOffset[0] - e.movementX * (1 / zoomLog);
    originOffset[1] = originOffset[1] - e.movementY * (1 / zoomLog);
    updateBounds();
    drawPoint();
  }
});
