import { Universe, Cell, init } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 5; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";
const MAX_SPEED = 500;

const universe = Universe.new();
const height = universe.height();
const width = universe.width();

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const slider = document.getElementById("speed-slider");
const sliderOutput = document.getElementById("speed-output");
let speed = MAX_SPEED - slider.value;
sliderOutput.textContent = slider.value;

let paused = false;

const ctx = canvas.getContext('2d');

const getIndex = (row, column) => {
    return row * width + column;
};

let animationId = null;

const sleep = (delay) => new Promise((resolve => setTimeout(resolve, delay)));

const renderLoop = async () => {
    universe.tick();

    drawGrid();
    drawCells();

    await sleep(speed);
    if (!paused) {
        animationId = requestAnimationFrame(renderLoop);
    }
};

const isPaused = () => {
    return animationId === null;
}

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical Lines
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    paintCells(DEAD_COLOR, Cell.Dead, cells);
    paintCells(ALIVE_COLOR, Cell.Alive, cells);
    ctx.stroke();
};

const paintCells = (colour, cellState, cells) => {
    ctx.fillStyle = colour;
    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);
            if (cells[idx] === cellState) {
                ctx.fillRect(
                    col * (CELL_SIZE + 1) + 1,
                    row * (CELL_SIZE + 1) + 1,
                    CELL_SIZE,
                    CELL_SIZE
                );
            }
        }
    }

}

const playPauseButton = document.getElementById("play-pause");
const clearButton = document.getElementById("clear");
clearButton.textContent = "💀";

const play = async () => {
    paused = false;
    playPauseButton.textContent = "⏸";
    await renderLoop();
};

const pause = () => {
    paused = true;
    playPauseButton.textContent = "▶️";
    cancelAnimationFrame(animationId);
    animationId = null;
};

playPauseButton.addEventListener("click", async event => {
    if (isPaused()) {
        await play();
    } else {
        pause();
    }
});

clearButton.addEventListener("click", event => {
    universe.clear();
    drawGrid();
    drawCells();
})

slider.addEventListener("input", event => {
    speed = MAX_SPEED - slider.value;
    sliderOutput.textContent = slider.value;
})

canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    universe.toggle_cell(row, col);

    drawGrid();
    drawCells();
})

init();
drawGrid();
drawCells();
play();