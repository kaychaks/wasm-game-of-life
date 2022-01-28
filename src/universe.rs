use crate::{cell::Cell, wasm_bindgen};
use fixedbitset::FixedBitSet;
use itertools::Itertools;
use js_sys::Math;

#[wasm_bindgen]
pub struct Universe {
    height: usize,
    width: usize,
    cells: FixedBitSet,
}

fn get_index(width: usize, row: usize, col: usize) -> usize {
    row * width + col
}

impl Universe {
    fn get_index(&self, row: usize, column: usize) -> usize {
        get_index(self.width, row, column)
    }

    fn live_neighbour_count(&self, row: usize, column: usize) -> u8 {
        [self.height - 1, 0, 1]
            .iter()
            .cloned()
            .cartesian_product([self.width - 1, 0, 1].iter().cloned())
            .fold(0_u8, |acc, (dr, dc)| {
                if dr == 0 && dc == 0 {
                    acc
                } else {
                    let nr = (dr + row) % self.height;
                    let nc = (dc + column) % self.width;
                    let idx = self.get_index(nr, nc);
                    acc + self.cells[idx] as u8
                }
            })
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    pub fn tick(&mut self) {
        self.cells = (0..self.height).cartesian_product(0..self.width).fold(
            self.cells.clone(),
            |mut acc, (row, col)| {
                let idx = self.get_index(row, col);
                let ln = self.live_neighbour_count(row, col);
                let cell = self.cells[idx].into();

                let nc = match (cell, ln) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                acc.set(idx, nc.into());
                acc
            },
        );
    }

    pub fn new(width: usize, height: usize) -> Universe {
        let mut cells = FixedBitSet::with_capacity(height * width);

        (0..height * width).for_each(|id| cells.set(id, Math::random() < 0.5));

        Universe {
            width,
            height,
            cells,
        }
    }
}
