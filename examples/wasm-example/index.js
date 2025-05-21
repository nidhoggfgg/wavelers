import init, { dwt2 } from "./pkg/wasm_example.js";

function displayImage(canvasId, data, width, height) {
  const canvas = document.getElementById(canvasId);
  canvas.width = width;
  canvas.height = height;
  const ctx = canvas.getContext('2d');

  let min = Infinity;
  let max = -Infinity;
  for (let i = 0; i < data.length; i++) {
    if (data[i] < min) min = data[i];
    if (data[i] > max) max = data[i];
  }
  const range = max - min;

  const imageData = ctx.createImageData(width, height);
  for (let i = 0; i < data.length; i++) {
    const value = Math.floor(((data[i] - min) / range) * 255);
    const idx = i * 4;
    imageData.data[idx] = value;     // R
    imageData.data[idx + 1] = value; // G
    imageData.data[idx + 2] = value; // B
    imageData.data[idx + 3] = 255;   // A
  }

  ctx.putImageData(imageData, 0, 0);
}

function main() {
  document.getElementById('imageInput').addEventListener('change', function (e) {
    const file = e.target.files[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = function (event) {
      const img = new Image();
      img.onload = function () {
        const canvas = document.createElement('canvas');
        const ctx = canvas.getContext('2d');

        const width = Math.pow(2, Math.floor(Math.log2(img.width)));
        const height = Math.pow(2, Math.floor(Math.log2(img.height)));
        const halfWidth = width >> 1;
        const halfHeight = height >> 1;

        canvas.width = width;
        canvas.height = height;
        ctx.drawImage(img, 0, 0, width, height);

        const imageData = ctx.getImageData(0, 0, width, height).data;
        const grayscale = new Float64Array(width * height);
        for (let i = 0; i < grayscale.length; i++) {
          const idx = i * 4;
          grayscale[i] = 0.299 * imageData[idx] +
            0.587 * imageData[idx + 1] +
            0.114 * imageData[idx + 2];
        }

        displayImage('originalCanvas', grayscale, width, height);

        let ll = new Float64Array(halfWidth * halfHeight);
        let lh = new Float64Array(halfWidth * halfHeight);
        let hl = new Float64Array(halfWidth * halfHeight);
        let hh = new Float64Array(halfWidth * halfHeight);
        dwt2(grayscale, width, height, ll, lh, hl, hh, "haar")

        displayImage('llCanvas', ll, halfWidth, halfHeight);
        displayImage('lhCanvas', lh, halfWidth, halfHeight);
        displayImage('hlCanvas', hl, halfWidth, halfHeight);
        displayImage('hhCanvas', hh, halfWidth, halfHeight);
      };
      img.src = event.target.result;
    };
    reader.readAsDataURL(file);
  });
}

async function run() {
  await init();
  main();
}
run();
