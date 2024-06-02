mod utils;

use std::fmt;

use wasm_bindgen::prelude::*;
extern crate js_sys;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}


#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();
        log!("Initializing Universe...");
        let width = 64;
        let height = 64;

        let cells = (0..width * height).map(|_| {
            if js_sys::Math::random() < 0.5 { 
                Cell::Alive
            } else {
                Cell::Dead
            }
        }).collect();

        return Universe {
            width,
            height,
            cells,
        };
    }

    pub fn width(&self) -> u32 {
        return self.width;
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_| Cell::Dead).collect();
    }

    pub fn height(&self) -> u32 {
        return self.height;
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_| Cell::Dead).collect();
    }

    pub fn cells(&self) -> *const Cell {
        return self.cells.as_ptr();
    }

    pub fn render(&self) -> String {
        return self.to_string();
    }

    fn get_index(&self, x: u32, y: u32) -> usize {
        return (x + y * self.width) as usize;
    }

    fn live_neighbour_count(&self, x: u32, y: u32) -> u8 {
        let mut count: u8 = 0;
        for delta_y in [self.height - 1, 0, 1].iter().cloned() {
            for delta_x in [self.width - 1, 0, 1].iter().cloned() {
                if delta_x == 0 && delta_y == 0 {
                    continue;
                }
                let neighbour_x = (x + delta_x) % self.width;
                let neighbour_y = (y + delta_y) % self.height;
                let idx = self.get_index(neighbour_x, neighbour_y);
                count += self.cells[idx] as u8;
            }
        }
        return count;
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.get_index(x, y);
                let cell = self.cells[index];
                let live_neighbours = self.live_neighbour_count(x, y);
                let next_cell = match (cell, live_neighbours) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                next[index] = next_cell;
            }
        }

        self.cells = next;
    }
}

impl Universe {
    pub fn get_cells(&self) -> &[Cell] {
        return &self.cells;
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (x, y) in cells.iter().cloned() {
            let idx = self.get_index(x, y);
            self.cells[idx] = Cell::Alive;
        }
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
