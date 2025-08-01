use crate::{NUM_COLUMNS, NUM_ROWS};
use juquad::widgets::anchor::{Horizontal, Vertical};
use macroquad::prelude::IVec2;

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
    pub fn horiz_rows(&self) -> i32 {
        self.horizontal.len() as i32
    }
    pub fn horiz_columns(&self) -> i32 {
        self.horizontal.get(0).unwrap().len() as i32
    }
    pub fn vert_rows(&self) -> i32 {
        self.vertical.len() as i32
    }
    pub fn vert_columns(&self) -> i32 {
        self.vertical.get(0).unwrap().len() as i32
    }
    pub fn get_horiz(&self, row: i32, column: i32) -> Horizontal {
        *self
            .horizontal
            .get(row as usize)
            .unwrap()
            .get(column as usize)
            .unwrap()
    }
    pub fn get_horiz_mut(&mut self, row: i32, column: i32) -> &mut Horizontal {
        self.horizontal
            .get_mut(row as usize)
            .unwrap()
            .get_mut(column as usize)
            .unwrap()
    }
    pub fn get_vert(&self, row: i32, column: i32) -> Vertical {
        *self
            .vertical
            .get(row as usize)
            .unwrap()
            .get(column as usize)
            .unwrap()
    }
    pub fn get_vert_mut(&mut self, row: i32, column: i32) -> &mut Vertical {
        self.vertical
            .get_mut(row as usize)
            .unwrap()
            .get_mut(column as usize)
            .unwrap()
    }
}

pub type Cell = bool;
pub struct Grid {
    pub cells: Vec<Vec<Cell>>,
    pub rails: Rails,
    pub root: IVec2,
}

impl Grid {
    pub fn new(num_rows: i32, num_columns: i32, root: IVec2) -> Self {
        let mut row = Vec::new();
        row.resize(num_columns as usize, false);
        let mut cells = Vec::new();
        cells.resize(num_rows as usize, row);

        let rails = Rails::new(num_rows, num_columns);

        let mut grid = Self { cells, rails, root };
        *get_mut(&mut grid, NUM_ROWS / 2, NUM_COLUMNS / 2) = true;
        grid
    }

    pub fn recalculate_rails(&mut self) {
        for i_row in 1..NUM_ROWS {
            for i_column in 1..NUM_COLUMNS {
                let current = *get(&self, i_row, i_column);
                let above = *get(&self, i_row - 1, i_column);
                let left = *get(&self, i_row, i_column - 1);

                let direction = if current != above {
                    if current {
                        Horizontal::Right
                    } else {
                        Horizontal::Left
                    }
                } else {
                    Horizontal::Center
                };
                *self.rails.get_horiz_mut(i_row, i_column) = direction;

                let direction = if current != left {
                    if current {
                        Vertical::Top
                    } else {
                        Vertical::Bottom
                    }
                } else {
                    Vertical::Center
                };
                *self.rails.get_vert_mut(i_row, i_column) = direction;
            }
        }
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
    row > 0 && row < NUM_ROWS - 1 && column > 0 && column < NUM_COLUMNS - 1
}
