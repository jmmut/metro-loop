use crate::SIZE;
use juquad::widgets::anchor::{Horizontal, Vertical};

pub struct Rails {
    horizontal: Vec<Vec<Horizontal>>,
    vertical: Vec<Vec<Vertical>>,
}

impl Rails {
    pub fn new(num_rows: i32, num_columns: i32) -> Self {
        let mut row = Vec::new();
        row.resize(num_columns as usize, Horizontal::Center);
        let mut horizontal = Vec::new();
        horizontal.resize(num_rows as usize + 1, row);

        let mut row = Vec::new();
        row.resize(num_columns as usize + 1, Vertical::Center);
        let mut vertical = Vec::new();
        vertical.resize(num_rows as usize, row);

        Self {
            horizontal,
            vertical,
        }
    }
}

pub type Grid = Vec<Vec<bool>>;
pub type Cell = bool;

pub fn get_mut(grid: &mut Grid, row: usize, column: usize) -> &mut Cell {
    grid.get_mut(row).unwrap().get_mut(column).unwrap()
}

pub fn get(grid: &Grid, row: usize, column: usize) -> &Cell {
    grid.get(row).unwrap().get(column).unwrap()
}

pub fn count_neighbours(grid: &Grid, row: usize, column: usize) -> i32 {
    *get(grid, row + 1, column) as i32
        + *get(grid, row - 1, column) as i32
        + *get(grid, row, column + 1) as i32
        + *get(grid, row, column - 1) as i32
}

pub fn in_range(row: usize, column: usize) -> bool {
    row > 0 && row < SIZE - 1 && column > 0 && column < SIZE - 1
}
