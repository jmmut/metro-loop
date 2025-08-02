use crate::rails::Rails;
use crate::{NUM_COLUMNS, NUM_ROWS};
use juquad::widgets::anchor::{Horizontal, Vertical};
use macroquad::prelude::IVec2;
use crate::intersection::Intersections;

pub type Cell = bool;
pub struct Grid {
    pub cells: Vec<Vec<Cell>>,
    pub rails: Rails,
    pub intersections: Intersections,
    pub root: IVec2,
}

impl Grid {
    pub fn new(num_rows: i32, num_columns: i32, root: IVec2) -> Self {
        let mut row = Vec::new();
        row.resize(num_columns as usize, false);
        let mut cells = Vec::new();
        cells.resize(num_rows as usize, row);

        let rails = Rails::new(num_rows, num_columns);
        let intersections = Intersections::new(num_rows, num_columns);

        let mut grid = Self { cells, rails, root, intersections };
        *get_mut(&mut grid, NUM_ROWS / 2, NUM_COLUMNS / 2) = true;
        grid
    }
    pub fn rows(&self) -> i32 {
        self.cells.len() as i32
    }
    pub fn columns(&self) -> i32 {
        self.cells.get(0).unwrap().len() as i32
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
