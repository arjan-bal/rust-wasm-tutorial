import { Universe, Cell } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg"

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.new();
const height = universe.height()
const width = universe.width();

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

const renderLoop = () => {
  universe.tick();
  drawGrid();
  drawCells();

  requestAnimationFrame(renderLoop);
  // setTimeout(() => requestAnimationFrame(renderLoop), 100);
};

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines
  for (let i = 0; i <= width; i++) {
    ctx.moveTo((CELL_SIZE + 1) * i + 1, 0);
    ctx.lineTo((CELL_SIZE + 1) * i + 1, (CELL_SIZE + 1) * height + 1)
  }

  // Horizontal lines
  for (let i = 0; i <= height; i++) {
    ctx.moveTo(0, (CELL_SIZE + 1) * i + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, (CELL_SIZE + 1) * i + 1);
  }

  ctx.stroke();
}

const getIndex = (row, column) => {
  return row * width + column;
}

const drawCells = () => {
  const cellsPtr = universe.cells();
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

  ctx.beginPath();
  for (let i = 0; i < height; i++) {
    for (let j = 0; j < width; j++) {
      const idx = getIndex(i, j);
      ctx.fillStyle = cells[idx] === Cell.Alive ? ALIVE_COLOR : DEAD_COLOR;
      ctx.fillRect(
        (CELL_SIZE + 1) * j + 1,
        (CELL_SIZE + 1) * i + 1,
        CELL_SIZE,
        CELL_SIZE
      );
    }
  }

  ctx.stroke();
}

requestAnimationFrame(renderLoop);