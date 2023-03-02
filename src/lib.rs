mod utils;

use std::fmt::Display;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone)]
pub enum Cell {
    Dead,
    Alive,
}

#[wasm_bindgen]
pub struct Universe {
    height: usize,
    width: usize,
    cells: Vec<Cell>,
}

impl Universe {
    fn live_neighbours_count(&self, c_i: usize, c_j: usize) -> u8 {
        let mut count = 0;
        for delta_i in [self.height - 1, 0, 1].iter() {
            for delta_j in [self.width - 1, 0, 1].iter() {
                if delta_i == delta_j {
                    continue;
                }
                let ni = (c_i + delta_i) % self.height;
                let nj = (c_j + delta_j) % self.width;
                if let Cell::Alive = self.cells[self.get_1d_coordinate(ni, nj)] {
                    count += 1;
                }
            }
        }
        count
    }

    fn get_1d_coordinate(&self, ci: usize, cj: usize) -> usize {
        ci * self.width + cj
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = vec![Cell::Dead; self.height * self.width];
        for i in 0..self.height {
            for j in 0..self.width {
                let live_neighbours = self.live_neighbours_count(i, j);
                let index = self.get_1d_coordinate(i, j);
                next[index] = match (&self.cells[index], live_neighbours) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, x) if x >= 2 && x <= 3 => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    _ => Cell::Dead,
                };
            }
        }
        self.cells = next;
    }

    pub fn new() -> Universe {
        let height = 64;
        let width = 64;
        let cells = (0..(height * width))
            .map(|index| {
                if index % 7 == 0 || index % 2 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        Universe {
            height,
            width,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn height(&self) -> u32 {
        self.height as u32
    }

    pub fn width(&self) -> u32 {
        self.width as u32
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.cells.iter().as_slice().chunks(self.width) {
            for col in row.iter() {
                write!(
                    f,
                    "{}",
                    match col {
                        Cell::Alive => '◼',
                        Cell::Dead => '◻',
                    }
                )?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
