use crate::logic::grid::{get_cell, Grid};
use crate::CLUE_PERCENTAGE;
use juquad::widgets::anchor::{Horizontal, Vertical};
use macroquad::rand::rand;

#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub struct Constraints {
    pub rails: Vec<RailCoord>,
    pub cell_count: i32,
}

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub struct Satisfaction {
    pub failing_rails: i32,
    pub cell_diff: i32,
    pub unconnected_loops: i32,
    pub unreachable_rails: i32,
}
impl Satisfaction {
    pub fn success(&self) -> bool {
        self.failing_rails == 0 && self.cell_diff == 0
            // && self.unconnected_loops == 0
        && self.unreachable_rails == 0
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
            if rand() % 100 < CLUE_PERCENTAGE && !is_root {
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
            if rand() % 100 < CLUE_PERCENTAGE {
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
    // let unconnected_loops = (count_loops(grid) - 1).abs();
    let unconnected_loops = 1;
    let unreachable_rails = count_unreachable_rails(grid);
    Satisfaction {
        failing_rails,
        cell_diff,
        unconnected_loops,
        unreachable_rails,
    }
}

pub fn count_unreachable_rails(grid: &Grid) -> i32 {
    grid.total_rails - grid.reachable_rails_count
}
pub fn count_loops(grid: &Grid) -> i32 {
    let active = count_cells(grid);
    let mut adjacents = 0;
    for row in 0..grid.rows() {
        for column in 0..grid.columns() {
            let current = *get_cell(grid, row, column);
            if row > 0 {
                let above = *get_cell(grid, row - 1, column);
                adjacents += (current && above) as i32;
            }
            if column > 0 {
                let left = *get_cell(grid, row, column - 1);
                adjacents += (current && left) as i32;
            }
            if row > 0 && column > 0 {
                let left = *get_cell(grid, row, column - 1);
                let above = *get_cell(grid, row - 1, column);
                let above_left = *get_cell(grid, row - 1, column - 1);
                adjacents -= (current && left && above && above_left) as i32;
            }
        }
    }
    active - adjacents
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

#[derive(PartialEq)]
pub enum Reverse {
    Reverse,
    None,
    Regular,
}
impl Reverse {
    pub fn is_reverse(&self) -> bool {
        *self == Reverse::Reverse
    }
}
impl From<Horizontal> for Reverse {
    fn from(value: Horizontal) -> Self {
        match value {
            Horizontal::Left => Reverse::Reverse,
            Horizontal::Center => Reverse::None,
            Horizontal::Right => Reverse::Regular,
        }
    }
}
impl From<Vertical> for Reverse {
    fn from(value: Vertical) -> Self {
        match value {
            Vertical::Top => Reverse::Reverse,
            Vertical::Center => Reverse::None,
            Vertical::Bottom => Reverse::Regular,
        }
    }
}
pub fn matches_constraint_and_reachable(grid: &Grid, constraint: &RailCoord) -> (bool, Reverse) {
    match *constraint {
        RailCoord::Horizontal {
            row,
            column,
            direction,
        } => {
            let rail = grid.rails.get_horiz(row, column);
            (
                rail == direction
                    && (direction == Horizontal::Center
                        || grid.reachable_rails.get_horiz(row, column)),
                rail.into(),
            )
        }
        RailCoord::Vertical {
            row,
            column,
            direction,
        } => {
            let rail = grid.rails.get_vert(row, column);
            (
                rail == direction
                    && (direction == Vertical::Center
                        || grid.reachable_rails.get_vert(row, column)),
                rail.into(),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::grid::Cell;
    use crate::logic::intersection::Intersections;
    use crate::logic::rails::Rails;
    use macroquad::prelude::IVec2;

    fn mock_grid(cells: Vec<Vec<Cell>>) -> Grid {
        let rails = Rails::new(0, 0, Horizontal::Center, Vertical::Center);
        let reachable_rails = Rails::new(0, 0, false, false);
        let fixed_rails = Rails::new(0, 0, false, false);
        let root = IVec2::default();
        let intersections = Intersections::new(0, 0);
        let fixed_cells = cells.clone();
        Grid {
            num_rows: cells.len() as i32,
            num_columns: cells.first().unwrap().len() as i32,
            cells,
            fixed_cells,
            rails,
            reachable_rails,
            fixed_rails,
            root,
            intersections,
            total_rails: 0,
            reachable_rails_count: 0,
        }
    }
    const CLICK: bool = true;

    #[test]
    fn test_count_loops_none() {
        let grid = mock_grid(vec![
            vec![false, false, false],
            vec![false, false, false],
            vec![false, false, false],
        ]);
        let loops = count_loops(&grid);
        assert_eq!(loops, 0);
    }

    #[test]
    fn test_count_loops_one() {
        let grid = mock_grid(vec![
            vec![false, false, false],
            vec![false, CLICK, false],
            vec![false, false, false],
        ]);
        let loops = count_loops(&grid);
        assert_eq!(loops, 1);
    }
    #[test]
    fn test_count_loops_two() {
        let grid = mock_grid(vec![
            vec![false, false, false, false, false],
            vec![false, CLICK, false, CLICK, false],
            vec![false, false, false, false, false],
        ]);
        let loops = count_loops(&grid);
        assert_eq!(loops, 2);
    }
    #[test]
    fn test_count_loops_border() {
        #[rustfmt::skip]
        let grid = mock_grid(vec![
            vec![false, CLICK],
            vec![false, false],
        ]);
        let loops = count_loops(&grid);
        assert_eq!(loops, 1);
    }
    #[test]
    fn test_count_loops_border_big() {
        #[rustfmt::skip]
        let grid = mock_grid(vec![
            vec![false, CLICK, CLICK],
            vec![false, false, false],
        ]);
        let loops = count_loops(&grid);
        assert_eq!(loops, 1);
    }
    #[test]
    fn test_count_loops_top_left() {
        #[rustfmt::skip]
        let grid = mock_grid(vec![
            vec![CLICK, false],
            vec![false, false],
        ]);
        let loops = count_loops(&grid);
        assert_eq!(loops, 1);
    }
    #[test]
    fn test_count_loops_one_big() {
        let grid = mock_grid(vec![
            vec![false, false, CLICK, false, false],
            vec![false, CLICK, CLICK, CLICK, false],
            vec![false, CLICK, false, false, false],
        ]);
        let loops = count_loops(&grid);
        assert_eq!(loops, 1);
    }
    #[test]
    fn test_count_loops_two_big() {
        let grid = mock_grid(vec![
            vec![false, false, CLICK, false, false],
            vec![false, CLICK, CLICK, CLICK, false],
            vec![CLICK, false, false, false, false],
        ]);
        let loops = count_loops(&grid);
        assert_eq!(loops, 2);
    }
    #[test]
    fn test_count_loops_one_square() {
        let grid = mock_grid(vec![
            vec![false, false, false, false, false],
            vec![false, CLICK, CLICK, CLICK, false],
            vec![false, CLICK, CLICK, CLICK, false],
        ]);
        let loops = count_loops(&grid);
        assert_eq!(loops, 1);
    }
    #[test]
    fn test_count_loops_one_donut() {
        let grid = mock_grid(vec![
            vec![false, false, false, false, false],
            vec![false, CLICK, CLICK, CLICK, false],
            vec![false, CLICK, false, CLICK, false],
            vec![false, CLICK, CLICK, CLICK, false],
        ]);
        let loops = count_loops(&grid);
        assert_eq!(loops, 0); // arguably, the inner circuit is a separate loop, counted as negative
    }
}
