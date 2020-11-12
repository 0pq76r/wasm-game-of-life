mod utils;

use wasm_bindgen::prelude::*;
use embedded_graphics::fonts::{Font,Font6x8};
use quad_rand as qrand;
use std::fmt;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}

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

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (column + delta_col) % self.width;
//                //deadly border
//                if ((neighbor_row as i64) - (row as i64)).abs() <= 1 &&
//                    ((neighbor_col as i64) - (column as i64)).abs() <= 1
                {
                    let idx = self.get_index(neighbor_row, neighbor_col);
                    count += self.cells[idx] as u8;
                }
            }
        }
        count
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn printLetter(&mut self, e:u32, c: char) {
        let mut next = self.cells.clone();
        for x in 0..6 {
            for y in 0..8 {
                let col = 3+5*(6*e+x);
                let row = 3+5*y;
                if Font6x8::character_pixel(c, x as u32, y as u32) {
                    next[(((row+self.height-0)%self.height)*self.width + ((col+self.width-0)%self.width)) as usize] = Cell::Alive;
                    next[(((row+self.height-0)%self.height)*self.width + ((col+self.width-1)%self.width)) as usize] = Cell::Alive;
                    next[(((row+self.height-0)%self.height)*self.width + ((col+self.width-2)%self.width)) as usize] = Cell::Alive;
                    next[(((row+self.height-1)%self.height)*self.width + ((col+self.width-2*(e%2))%self.width)) as usize] = Cell::Alive;
                    next[(((row+self.height-2)%self.height)*self.width + ((col+self.width-1)%self.width)) as usize] = Cell::Alive;
                }
            }
        }
        self.cells = next;
    }
    
    pub fn tick(&mut self) {
        // copy universe's cells
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);
                let next_cell = match (cell, live_neighbors, qrand::gen_range(0., 1.) < 0.0f64) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x, _) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2, _) | (Cell::Alive, 3, _) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x, _) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3, _) => Cell::Alive,
                    // Random walker
                    (_, _, true) => {
                        next[self.get_index((row+self.height-0)%self.height,(col+self.width-1)%self.width)] = Cell::Alive;
                        next[self.get_index((row+self.height-0)%self.height,(col+self.width-2)%self.width)] = Cell::Alive;                        
                        next[self.get_index((row+self.height-1)%self.height,(col+self.width-2)%self.width)] = Cell::Alive;
                        next[self.get_index((row+self.height-2)%self.height,(col+self.width-1)%self.width)] = Cell::Alive;
                        Cell::Alive},
                    // All other cells remain in the same state.
                    (otherwise, _, _) => otherwise,
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }
    pub fn new(width:usize, height:usize) -> Universe {
        let cells: Vec<Cell> = (0..width * height)
            .map(|_| {
                Cell::Dead
            })
            .collect();

        let mut u = Universe {
            width:(width as u32),
            height:(height as u32),
            cells,
        };

        for (e,c) in "It works!".chars().enumerate()
        {
            u.printLetter(e as u32,c)
        }

        u
    }
    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { ' ' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

