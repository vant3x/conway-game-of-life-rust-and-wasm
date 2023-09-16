mod utils;

use wasm_bindgen::prelude::*;
use std::fmt;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello {} {}", name, "game of life  with wasm and rust"));
}

#[wasm_bindgen]
#[repr(u8)] 
#[derive(Clone, Copy, Debug, PartialEq, Eq)]

pub enum Cell {
    Dead = 0,
    Alive = 1,
}
#[wasm_bindgen]
pub struct  Universe {
    width: u32, 
    heigth: u32,
    cells: Vec<Cell>
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn live_neightbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.heigth - 1, 0, 1].iter().cloned() {
           for delta_col in [self.width - 1, 0, 1].iter().cloned() {
            if delta_row == 0 && delta_col == 0 {
                continue;
            } 
            let neighbor_row = (row + delta_row) % self.heigth;
            let neighbor_col = (column + delta_col) % self.width;
            let idx = self.get_index(neighbor_row, neighbor_col);
            count += self.cells[idx] as u8;
        }
    }
    count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Regla 1: Cualquier célula viva con menos de dos vecinas vivas
                    // muere, como si se tratara de una subpoblación.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                     // Regla 2: Cualquier célula viva con dos o tres vecinas vivas
                    // vive en la siguiente generación.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Regla 3: Cualquier célula viva con más de tres vecinas vivas
                    // vecinas mueren, como por superpoblación.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                     // Regla 4: Cualquier célula muerta con exactamente tres vecinas vivas
                    // se convierte en célula viva, como por reproducción.
                    (Cell::Dead, 3) => Cell::Alive,
                    // todas las demás se mantienen en el mismo estado
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}


impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}