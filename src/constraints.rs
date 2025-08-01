use crate::rails::Grid;
use juquad::widgets::anchor::{Horizontal, Vertical};
use macroquad::rand::rand;

pub enum RailCoord {
    Horizontal {
        row: i32,
        column: i32,
        direction: Horizontal,
    },
    Vertical {
        row: i32,
        column: i32,
        direction: Vertical,
    },
}

pub struct Constraints {
    pub rails: Vec<RailCoord>,
    pub cell_count: i32,
}

pub struct Satisfaction {
    pub failing_rails: i32,
    pub cell_diff: i32,
    pub unconnected_loops: i32,
}
impl Satisfaction {
    pub fn success(&self) -> bool {
        self.failing_rails == 0 && self.cell_diff == 0 && self.unconnected_loops == 0
    }
}
pub fn choose_constraints(grid: &Grid) -> Constraints {
    let mut rails = Vec::new();
    {
        let row = grid.root.y;
        let column = grid.root.x;
        let direction = grid.rails.get_horiz(row, column);
        rails.push(RailCoord::Horizontal {
            row,
            column,
            direction,
        });
    }
    for row in 1..grid.rails.horiz_rows() - 1 {
        for column in 1..grid.rails.horiz_columns() - 1 {
            let is_root = row == grid.root.y && column == grid.root.x; // avoid adding the root twice
            if rand() % 100 < 30 && !is_root {
                let direction = grid.rails.get_horiz(row, column);
                rails.push(RailCoord::Horizontal {
                    row,
                    column,
                    direction,
                });
            }
        }
    }
    for row in 1..grid.rails.vert_rows() - 1 {
        for column in 1..grid.rails.vert_columns() - 1 {
            if rand() % 100 < 30 {
                let direction = grid.rails.get_vert(row, column);
                rails.push(RailCoord::Vertical {
                    row,
                    column,
                    direction,
                });
            }
        }
    }
    let cell_count = count_cells(grid);
    Constraints { rails, cell_count }
}

fn count_cells(grid: &Grid) -> i32 {
    let mut cell_count = 0;
    for row in &grid.cells {
        for cell in row {
            if *cell {
                cell_count += 1;
            }
        }
    }
    cell_count
}

pub fn compute_satisfaction(grid: &Grid, constraints: &Constraints) -> Satisfaction {
    let failing_rails = compute_rail_failures(grid, &constraints.rails);
    let cell_diff = constraints.cell_count - count_cells(grid);
    let unconnected_loops = 0; // TODO
    Satisfaction {
        failing_rails,
        cell_diff,
        unconnected_loops,
    }
}

fn compute_rail_failures(grid: &Grid, rail_constraints: &Vec<RailCoord>) -> i32 {
    let mut failures = 0;
    for constraint in rail_constraints {
        if !matches_constraint(grid, constraint) {
            failures += 1;
        }
    }
    failures
}

pub fn matches_constraint(grid: &Grid, constraint: &RailCoord) -> bool {
    match *constraint {
        RailCoord::Horizontal {
            row,
            column,
            direction,
        } => grid.rails.get_horiz(row, column) == direction,
        RailCoord::Vertical {
            row,
            column,
            direction,
        } => grid.rails.get_vert(row, column) == direction,
    }
}
