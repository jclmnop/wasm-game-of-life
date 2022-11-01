mod utils;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use wasm_bindgen::prelude::*;
use js_sys::Math::random;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();
}

/// Represents a single cell, either dead or alive
#[wasm_bindgen]
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead  => Cell::Alive,
            Cell::Alive => Cell::Dead,
        }
    }

}

/// Represents the Universe where all cells live
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    next_cells: Vec<Cell>,
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
        self.next_generation();
        self.cells = self.next_cells.clone();
    }

    /// Render current universe state as text
    pub fn render(&self) -> String {
        self.to_string()
    }

    /// Width of the universe (columns)
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Height of the universe (rows)
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Pointer to the first cell in array of all cells
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    /// Toggle state of cell between dead and alive
    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        self.cells[idx].toggle();
    }

    /// Set the width of the universe
    ///
    /// All cells will die
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.kill_all_cells();
    }

    /// Set the height of the universe
    ///
    /// All cells will DIE
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.kill_all_cells();
    }

    /// Kills all cells
    pub fn clear(&mut self) {
        self.kill_all_cells();
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

    fn next_generation(&mut self) {
        // Cloned because we need to refer to previous gen while calculating
        // next gen anyway
        self.next_cells = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbour_count(row, col);

                self.next_cells[idx] = match (cell, live_neighbours) {
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


    }

    fn kill_all_cells(&mut self) {
        self.cells = (0..self.height * self.width).map(|_| Cell::Dead).collect();
    }
}

/// For Rust testing
impl Universe {
    /// Get all cells
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Pass in array of (row, column) to set cells at those coordinates to
    /// be alive
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }


}

impl Default for Universe {
    fn default() -> Self {
        let width = 100;
        let height = 100;
        let cells: Vec<Cell> = (0..width * height).map(|_| {
            match random().total_cmp(&0.6) {
                Ordering::Greater => Cell::Alive,
                Ordering::Less    => Cell::Dead,
                Ordering::Equal   => Cell::Dead,
            }
        }).collect();

        Universe {
            width,
            height,
            next_cells: cells.clone(),
            cells,
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

