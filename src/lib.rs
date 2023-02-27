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
    cells: Vec<Vec<Cell>>,
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
                if let Cell::Alive = self.cells[ni][nj] {
                    count += 1;
                }
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = vec![vec![Cell::Dead; self.width]; self.height];
        for i in 0..self.height {
            for j in 0..self.width {
                let live_neighbours = self.live_neighbours_count(i, j);
                next[i][j] = match (&self.cells[i][j], live_neighbours) {
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
        let cells = (0..height)
            .map(|ci| {
                (0..width)
                    .map(|cj| {
                        if (ci * width + cj) % 7 == 0 || (ci + cj) % 2 == 0 {
                            Cell::Alive
                        } else {
                            Cell::Dead
                        }
                    })
                    .collect()
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
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.cells.iter() {
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
