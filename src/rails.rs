use crate::SIZE;
use juquad::widgets::anchor::{Horizontal, Vertical};

pub struct Rails {
    pub horizontal: Vec<Vec<Horizontal>>,
    pub vertical: Vec<Vec<Vertical>>,
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

pub type Cell = bool;
pub struct Grid {
    cells: Vec<Vec<Cell>>,
    rails: Rails,
}

impl Grid {
    pub fn new(num_rows: i32, num_columns: i32) -> Self {
        let mut row = Vec::new();
        row.resize(num_columns as usize, false);
        let mut cells = Vec::new();
        cells.resize(num_rows as usize, row);

        let rails = Rails::new(num_rows, num_columns);
        Self { cells, rails }
    }
}

pub fn get_mut(grid: &mut Grid, row: i32, column: i32) -> &mut Cell {
    assert!(row >= 0);
    assert!(column >= 0);
    grid.cells
        .get_mut(row as usize)
        .unwrap()
        .get_mut(column as usize)
        .unwrap()
}

pub fn get(grid: &Grid, row: i32, column: i32) -> &Cell {
    assert!(row >= 0);
    assert!(column >= 0);
    grid.cells
        .get(row as usize)
        .unwrap()
        .get(column as usize)
        .unwrap()
}

pub fn count_neighbours(grid: &Grid, row: i32, column: i32) -> i32 {
    *get(grid, row + 1, column) as i32
        + *get(grid, row - 1, column) as i32
        + *get(grid, row, column + 1) as i32
        + *get(grid, row, column - 1) as i32
}

pub fn in_range(row: i32, column: i32) -> bool {
    row > 0 && row < SIZE - 1 && column > 0 && column < SIZE - 1
}
