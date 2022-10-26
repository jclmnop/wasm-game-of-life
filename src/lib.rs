mod utils;

use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Represents a single cell, either dead or alive
#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

/// Represents the Universe where all cells live
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

/// Public Universe methods, exported to JS
#[wasm_bindgen] // These will be exported to JS so we use wasm_bindgen
impl Universe {
    /// Initialize a new universe with an interesting pattern of live and dead
    /// cells
    pub fn new() -> Universe {
        Universe::default()
    }

    /// Calculates next generation of cells and updates from previous
    /// generation to next generation.
    pub fn tick(&mut self) {
        self.cells = self.next_generation();
    }

    /// Render current universe state as text
    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neighbour_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        // Because we're using a "wrapping" grid, where the right neighbour of
        // a cell on the right edge of the universe is the neighbour at the very
        // start of that row, we use deltas and modulo instead of absolute
        // coordinates. Adding self.height/width - 1 (instead of just - 1) to 0
        // prevents underflow.
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue
                }

                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_col = (column + delta_col) % self.width;
                let idx = self.get_index(neighbour_row, neighbour_col);
                count += self.cells[idx] as u8; // repr[u8] lets us treat the enum as a u8
            }
        }
        count
    }

    fn next_generation(&self) -> Vec<Cell> {
        // Cloned because we need to refer to previous gen while calculating
        // next gen anyway
        let mut next_generation = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbour_count(row, col);

                next_generation[idx] = match (cell, live_neighbours) {
                    // Rule 1: Underpopulation
                    (Cell::Alive, n) if n < 2           => Cell::Dead,
                    // Rule 2: Stable population
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Overpopulation
                    (Cell::Alive, n) if n > 3           => Cell::Dead,
                    // Rule 4: Reproduction
                    (Cell::Dead, 3)                     => Cell::Alive,
                    // Everything else remains the same
                    (last_cell_state, _)                => last_cell_state,
                }
            }
        }

        next_generation
    }
}

impl Default for Universe {
    fn default() -> Self {
        let width = 64;
        let height = 64;
        let cells = (0..width * height).map(|i| {
            if i % 2 == 0 || i % 7 == 0 {
                Cell::Alive
            } else {
                Cell::Dead
            }
        }).collect();

        Universe {
            width,
            height,
            cells
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for row in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in row {
                let cell_symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{} ", cell_symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

