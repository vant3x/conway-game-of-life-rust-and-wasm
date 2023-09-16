mod utils;

use wasm_bindgen::prelude::*;

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

impl  Universe {
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for col in  0..self.width {
            let idx = self.get_index(row, col);
            let cell = self.cells[idx];
        }

    }
}