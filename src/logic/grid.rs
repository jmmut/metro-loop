use crate::logic::intersection::{
    crossing_to_char, horiz_to_char, vert_to_char, Crossing, Intersection, Intersections,
};
use crate::logic::rails::Rails;
use crate::{generate_nested_vec, AnyError};
use juquad::widgets::anchor::{Horizontal, Vertical};
use macroquad::prelude::{ivec2, IVec2};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub type Cell = bool;
#[derive(Clone)]
pub struct Grid {
    pub num_rows: i32,
    pub num_columns: i32,
    pub cells: Vec<Vec<Cell>>,
    pub fixed_cells: Vec<Vec<Cell>>,
    pub rails: Rails<Horizontal, Vertical>,
    pub reachable_rails: Rails<bool, bool>,
    pub fixed_rails: Rails<bool, bool>,
    pub intersections: Intersections,
    pub root: IVec2,
    pub total_rails: i32,
    pub reachable_rails_count: i32,
}

impl Grid {
    pub fn new(num_rows: i32, num_columns: i32, root: IVec2) -> Self {
        let mut cells = generate_nested_vec(num_rows as usize, num_columns as usize, false);
        let mut fixed_cells = generate_nested_vec(num_rows as usize, num_columns as usize, false);

        *get_mut(&mut cells, root.y, root.x) = true;
        *get_mut(&mut fixed_cells, root.y, root.x) = true;
        *get_mut(&mut fixed_cells, root.y - 1, root.x) = true;
        for i_row in 0..num_rows {
            *get_mut(&mut fixed_cells, i_row, 0) = true;
            *get_mut(&mut fixed_cells, i_row, num_columns - 1) = true;
        }
        for i_column in 0..num_columns {
            *get_mut(&mut fixed_cells, 0, i_column) = true;
            *get_mut(&mut fixed_cells, num_rows - 1, i_column) = true;
        }
        Self::new_from_cells(num_rows, num_columns, root, cells, fixed_cells)
    }
    pub fn new_from_cells(
        num_rows: i32,
        num_columns: i32,
        root: IVec2,
        cells: Vec<Vec<Cell>>,
        fixed_cells: Vec<Vec<Cell>>,
    ) -> Grid {
        let rails = Rails::new(num_rows, num_columns, Horizontal::Center, Vertical::Center);
        let reachable_rails = Rails::new(num_rows, num_columns, false, false);
        let fixed_rails = Rails::new(num_rows, num_columns, false, false);
        let intersections = Intersections::new(num_rows, num_columns);
        Self {
            num_rows,
            num_columns,
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

    pub fn rows(&self) -> i32 {
        self.num_rows
    }
    pub fn columns(&self) -> i32 {
        self.num_columns
    }
    pub fn recalculate_rails(&mut self) {
        let mut rail_count = 0;
        for i_row in 1..self.rows() {
            for i_column in 1..self.columns() {
                let current = *get_cell(&self, i_row, i_column);
                let above = *get_cell(&self, i_row - 1, i_column);
                let left = *get_cell(&self, i_row, i_column - 1);

                let direction = if current != above {
                    if current {
                        rail_count += 1;
                        Horizontal::Right
                    } else {
                        rail_count += 1;
                        Horizontal::Left
                    }
                } else {
                    Horizontal::Center
                };
                *self.reachable_rails.get_horiz_mut(i_row, i_column) = false;
                *self.rails.get_horiz_mut(i_row, i_column) = direction;

                let direction = if current != left {
                    if current {
                        rail_count += 1;
                        Vertical::Top
                    } else {
                        rail_count += 1;
                        Vertical::Bottom
                    }
                } else {
                    Vertical::Center
                };
                *self.reachable_rails.get_vert_mut(i_row, i_column) = false;
                *self.rails.get_vert_mut(i_row, i_column) = direction;
            }
        }
        for i_row in 1..self.intersections.rows() - 1 {
            for i_column in 1..self.intersections.columns() - 1 {
                let cell_current = *get_cell(&self, i_row, i_column);
                let cell_above = *get_cell(&self, i_row - 1, i_column);
                let cell_left = *get_cell(&self, i_row, i_column - 1);
                let cell_left_above = *get_cell(&self, i_row - 1, i_column - 1);

                let enabled_cells = cell_current as i32
                    + cell_left_above as i32
                    + cell_above as i32
                    + cell_left as i32;
                let crossing = if cell_current && cell_left_above && !cell_above && !cell_left {
                    Crossing::TopRightToBottomLeft
                } else if !cell_current && !cell_left_above && cell_above && cell_left {
                    Crossing::TopLeftToBottomRigt
                } else if [1, 2, 3].contains(&enabled_cells) {
                    Crossing::Single
                } else {
                    Crossing::None
                };
                *self.intersections.get_mut(i_row, i_column) = Intersection { crossing }
            }
        }
        let mut iterations = 0;
        let mut rail_coord = self.root;
        let mut rail_is_horizontal = true;
        let mut backwards = false;

        let print_debug = false;

        loop {
            iterations += 1;
            if iterations > 10000 {
                panic!("potential infinite loop in recalculate rails");
            }
            let row = rail_coord.y;
            let column = rail_coord.x;

            if print_debug {
                println!(
                    "row={}, column={}, rail horizontal is {}, \n{}",
                    row,
                    column,
                    rail_is_horizontal,
                    rails_to_string(&self)
                );
            }
            rail_coord = if rail_is_horizontal {
                *self.reachable_rails.get_horiz_mut(row, column) = true;
                let horizontal = self.rails.get_horiz_mut(row, column);
                if backwards {
                    *horizontal = horizontal.opposite();
                }
                let horizontal = *horizontal;
                let next_crossing = match horizontal {
                    Horizontal::Left => rail_coord,
                    Horizontal::Center => panic!(),
                    Horizontal::Right => ivec2(column + 1, row),
                };
                let crossing = self.intersections.get_mut(next_crossing.y, next_crossing.x);
                match crossing.crossing {
                    Crossing::None => panic!(),
                    Crossing::Single => {
                        let mut above = self.rails.get_vert(next_crossing.y - 1, next_crossing.x);
                        let mut below = self.rails.get_vert(next_crossing.y, next_crossing.x);
                        let mut left = self.rails.get_horiz(next_crossing.y, next_crossing.x - 1);
                        let mut right = self.rails.get_horiz(next_crossing.y, next_crossing.x);
                        if backwards {
                            above = above.opposite();
                            below = below.opposite();
                            left = left.opposite();
                            right = right.opposite();
                        }
                        // if backwards {
                        if below == Vertical::Bottom {
                            rail_is_horizontal = !rail_is_horizontal;
                            next_crossing
                        } else if above == Vertical::Top {
                            rail_is_horizontal = !rail_is_horizontal;
                            ivec2(next_crossing.x, next_crossing.y - 1)
                        } else if left == horizontal && left == Horizontal::Left {
                            ivec2(next_crossing.x - 1, next_crossing.y)
                        } else if right == horizontal && right == Horizontal::Right {
                            next_crossing
                        } else {
                            panic!()
                        }
                    }
                    Crossing::TopLeftToBottomRigt => {
                        backwards = !backwards;
                        crossing.crossing = Crossing::HorizontalOnTop;
                        if horizontal == Horizontal::Left {
                            next_crossing - ivec2(1, 0)
                        } else {
                            next_crossing
                        }
                    }
                    Crossing::TopRightToBottomLeft => {
                        backwards = !backwards;
                        crossing.crossing = Crossing::HorizontalOnTop;
                        if horizontal == Horizontal::Left {
                            next_crossing - ivec2(1, 0)
                        } else {
                            next_crossing
                        }
                    }
                    Crossing::VerticalOnTop => {
                        backwards = !backwards;
                        if horizontal == Horizontal::Left {
                            next_crossing - ivec2(1, 0)
                        } else {
                            next_crossing
                        }
                    }
                    Crossing::HorizontalOnTop => {
                        backwards = !backwards;
                        next_crossing
                    } // panic?
                }
            } else {
                *self.reachable_rails.get_vert_mut(row, column) = true;
                let vertical = self.rails.get_vert_mut(row, column);
                if backwards {
                    *vertical = vertical.opposite();
                }
                let vertical = *vertical;
                let next_crossing = match vertical {
                    Vertical::Top => rail_coord,
                    Vertical::Center => panic!(),
                    Vertical::Bottom => ivec2(column, row + 1),
                };
                let crossing = self.intersections.get_mut(next_crossing.y, next_crossing.x);
                match crossing.crossing {
                    Crossing::None => panic!(),
                    Crossing::Single => {
                        let mut above = self.rails.get_vert(next_crossing.y - 1, next_crossing.x);
                        let mut below = self.rails.get_vert(next_crossing.y, next_crossing.x);
                        let mut left = self.rails.get_horiz(next_crossing.y, next_crossing.x - 1);
                        let mut right = self.rails.get_horiz(next_crossing.y, next_crossing.x);
                        if backwards {
                            above = above.opposite();
                            below = below.opposite();
                            left = left.opposite();
                            right = right.opposite();
                        }
                        if right == Horizontal::Right {
                            rail_is_horizontal = !rail_is_horizontal;
                            next_crossing
                        } else if left == Horizontal::Left {
                            rail_is_horizontal = !rail_is_horizontal;
                            ivec2(next_crossing.x - 1, next_crossing.y)
                        } else if above == vertical && above == Vertical::Top {
                            ivec2(next_crossing.x, next_crossing.y - 1)
                        } else if below == vertical && below == Vertical::Bottom {
                            next_crossing
                        } else {
                            panic!()
                        }
                    }
                    Crossing::TopLeftToBottomRigt => {
                        backwards = !backwards;
                        crossing.crossing = Crossing::VerticalOnTop;
                        if vertical == Vertical::Bottom {
                            next_crossing
                        } else {
                            next_crossing - ivec2(0, 1)
                        }
                    }
                    Crossing::TopRightToBottomLeft => {
                        backwards = !backwards;
                        crossing.crossing = Crossing::VerticalOnTop;
                        if vertical == Vertical::Bottom {
                            next_crossing
                        } else {
                            next_crossing - ivec2(0, 1)
                        }
                    }
                    Crossing::VerticalOnTop => {
                        backwards = !backwards;
                        next_crossing
                    } // panic?
                    Crossing::HorizontalOnTop => {
                        backwards = !backwards;
                        if vertical == Vertical::Bottom {
                            next_crossing
                        } else {
                            next_crossing - ivec2(0, 1)
                        }
                    }
                }
            };
            if rail_coord == self.root && rail_is_horizontal {
                break;
            }
        }
        self.total_rails = rail_count;
        self.reachable_rails_count = iterations;
    }
}

pub fn get_mut<T>(vec_vec: &mut Vec<Vec<T>>, row: i32, column: i32) -> &mut T {
    assert!(row >= 0);
    assert!(column >= 0);
    vec_vec
        .get_mut(row as usize)
        .unwrap()
        .get_mut(column as usize)
        .unwrap()
}

pub fn get<T>(vec_vec: &Vec<Vec<T>>, row: i32, column: i32) -> &T {
    assert!(row >= 0);
    assert!(column >= 0);
    vec_vec
        .get(row as usize)
        .unwrap()
        .get(column as usize)
        .unwrap()
}
pub fn get_cell(grid: &Grid, row: i32, column: i32) -> &Cell {
    get(&grid.cells, row, column)
}

pub fn get_cell_mut(grid: &mut Grid, row: i32, column: i32) -> &mut Cell {
    get_mut(&mut grid.cells, row, column)
}

pub fn count_neighbours(grid: &Grid, row: i32, column: i32) -> i32 {
    *get_cell(grid, row + 1, column) as i32
        + *get_cell(grid, row - 1, column) as i32
        + *get_cell(grid, row, column + 1) as i32
        + *get_cell(grid, row, column - 1) as i32
}

pub fn in_range(grid: &Grid, row: i32, column: i32) -> bool {
    row > 0 && row < grid.rows() - 1 && column > 0 && column < grid.columns() - 1
}

pub struct GridAndRails<'a> {
    grid: &'a Grid,
}
impl Display for GridAndRails<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.grid.rows() {
            for column in 0..self.grid.columns() {
                let inter = self.grid.intersections.get(row, column);
                write!(f, "{}", crossing_to_char(inter))?;
                // write!(f, "{}", right_to_char(inter.right))?;
                let horiz = self.grid.rails.get_horiz(row, column);
                write!(f, "{}", horiz_to_char(horiz))?;
                // let inter = self.intersections.get(row, column+1);
                // write!(f, "{}", left_to_char(inter.left))?;
            }
            let inter = self.grid.intersections.get(row, self.grid.columns());
            write!(f, "{}", crossing_to_char(inter))?;
            writeln!(f)?;
            // for column in 0..self.columns() +1 {
            //     let inter = self.intersections.get(row, column);
            //     write!(f, "{}   ", below_to_char(inter.below))?;
            // }
            // writeln!(f)?;
            for column in 0..self.grid.columns() {
                let vert = self.grid.rails.get_vert(row, column);
                write!(f, "{} ", vert_to_char(vert))?;
            }
            let vert = self.grid.rails.get_vert(row, self.grid.columns());
            write!(f, "{}", vert_to_char(vert))?;
            writeln!(f)?;
            // for column in 0..self.columns() +1 {
            //     let inter = self.intersections.get(row+1, column);
            //     write!(f, "{}   ", above_to_char(inter.above))?;
            // }
            // writeln!(f)?;
        }
        let row = self.grid.rows();
        for column in 0..self.grid.columns() {
            let inter = self.grid.intersections.get(row, column);
            write!(f, "{}", crossing_to_char(inter))?;
            // write!(f, "{}", right_to_char(inter.right))?;
            let horiz = self.grid.rails.get_horiz(row, column);
            write!(f, "{}", horiz_to_char(horiz))?;
            // let inter = self.intersections.get(row, column+1);
            // write!(f, "{}", left_to_char(inter.left))?;
        }
        let inter = self.grid.intersections.get(row, self.grid.columns());
        write!(f, "{}", crossing_to_char(inter))?;
        writeln!(f)?;
        Ok(())
    }
}

pub fn rails_to_string(grid: &Grid) -> String {
    GridAndRails { grid }.to_string()
}

const ROOT_ROW: &str = "root_row";
const ROOT_COLUMN: &str = "root_column";

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // write!(
        //     f,
        //     "{}={},{}={},\n",
        //     ROOT_ROW, self.root.y, ROOT_COLUMN, self.root.x
        // )?;
        for row in 0..self.rows() {
            for column in 0..self.columns() {
                let cell = get(&self.cells, row, column);
                let fixed_cell = get(&self.fixed_cells, row, column);
                let letter = if self.root.x == column && self.root.y == row {
                    '%'
                } else {
                    match (cell, fixed_cell) {
                        (true, true) => '@',
                        (true, false) => 'O',
                        (false, true) => '.',
                        (false, false) => ' ',
                    }
                };
                write!(f, "{}", letter)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    pub fn from_str(s: &str) -> Result<Grid, AnyError> {
        let mut max_columns = 0;
        let mut cells = Vec::new();
        let mut fixed_cells = Vec::new();
        let mut line_count = 0;
        let lines = s.lines();
        let mut root = None;
        for line in lines {
            line_count += 1;
            let mut cell_row = Vec::new();
            let mut fixed_cell_row = Vec::new();
            let mut letter_count = 0;
            for letter in line.chars() {
                letter_count += 1;
                let (is_root, cell, fixed_cell) = match letter {
                    '%' => (true, true, true),
                    '@' => (false, true, true),
                    'O' => (false, true, false),
                    '.' => (false, false, true),
                    ' ' => (false, false, false),
                    _ => {
                        return Err(format!(
                            "Wrong format for grid at (1-based) line {}, letter {}: {}",
                            line_count, letter_count, line
                        )
                        .into())
                    }
                };
                if is_root {
                    root = Some(ivec2(letter_count - 1, line_count - 1));
                }
                cell_row.push(cell);
                fixed_cell_row.push(fixed_cell);
            }
            if cell_row.len() > max_columns {
                max_columns = cell_row.len();
            }
            cells.push(cell_row);
            fixed_cells.push(fixed_cell_row);
        }
        let Some(root) = root else {
            return Err("missing a root cell ('%')".into());
        };
        for row in &mut cells {
            row.resize(max_columns, false);
        }
        for row in &mut fixed_cells {
            row.resize(max_columns, false);
        }
        Ok(Grid::new_from_cells(
            cells.len() as i32,
            max_columns as i32,
            root,
            cells,
            fixed_cells,
        ))
    }

    #[allow(unused)]
    fn parse_metadata_line(first_line: Option<&str>) -> Result<IVec2, AnyError> {
        if let Some(first_line) = first_line {
            let mut root_x = None;
            let mut root_y = None;
            let without_spaces = first_line.replace(' ', "");
            let pairs = without_spaces.split_terminator(',').collect::<Vec<_>>();
            for pair in pairs {
                let words = pair.split('=').collect::<Vec<_>>();
                if words.len() != 2 {
                    return Err(format!(
                        "expected 2 words but got {} words in '{}'",
                        words.len(),
                        pair
                    )
                    .into());
                }
                let name = words[0];
                let value = words[1];
                if name == ROOT_ROW {
                    root_y = Some(i32::from_str(value)?)
                } else if name == ROOT_COLUMN {
                    root_x = Some(i32::from_str(value)?);
                } else {
                    // future compatible: ignore unknown keys
                }
            }
            if root_y.is_none() {
                Err(format!("missing {}", ROOT_ROW).into())
            } else if root_x.is_none() {
                Err(format!("missing {}", ROOT_COLUMN).into())
            } else {
                Ok(ivec2(root_x.unwrap(), root_y.unwrap()))
            }
        } else {
            Err("empty grid string".into())
        }
    }
}

#[cfg(test)]
mod grid_serde_tests {
    use super::*;
    #[test]
    fn test_minimal() {
        let grid = Grid::new(2, 1, ivec2(0, 1));
        let s = grid.to_string();
        let parsed = Grid::from_str(&s).unwrap();
        assert_eq!(parsed.cells, grid.cells);
        assert_eq!(parsed.fixed_cells, grid.fixed_cells);
        assert_eq!(parsed.root, grid.root);
    }
    #[test]
    fn test_4_3() {
        let grid = Grid::new(4, 3, ivec2(1, 2));
        let s = grid.to_string();
        let parsed = Grid::from_str(&s).unwrap();
        assert_eq!(parsed.cells, grid.cells);
        assert_eq!(parsed.fixed_cells, grid.fixed_cells);
        assert_eq!(parsed.root, grid.root);
    }
    #[test]
    fn test_6_5() {
        let grid = Grid::new(6, 5, ivec2(2, 3));
        let s = grid.to_string();
        let parsed = Grid::from_str(&s).unwrap();
        assert_eq!(parsed.cells, grid.cells);
        assert_eq!(parsed.fixed_cells, grid.fixed_cells);
        assert_eq!(parsed.root, grid.root);
    }
    #[test]
    fn test_extra_enabled() {
        let mut grid = Grid::new(6, 5, ivec2(2, 3));
        *get_mut(&mut grid.cells, 1, 3) = true;
        *get_mut(&mut grid.fixed_cells, 2, 3) = true;
        *get_mut(&mut grid.cells, 2, 1) = true;
        *get_mut(&mut grid.fixed_cells, 2, 1) = true;
        let s = grid.to_string();
        let parsed = Grid::from_str(&s).unwrap();
        assert_eq!(parsed.cells, grid.cells);
        assert_eq!(parsed.fixed_cells, grid.fixed_cells);
        assert_eq!(parsed.root, grid.root);
    }
    #[test]
    fn test_custom_root() {
        let grid = Grid::new(6, 5, ivec2(1, 3));
        let s = grid.to_string();
        let parsed = Grid::from_str(&s).unwrap();
        assert_eq!(parsed.cells, grid.cells);
        assert_eq!(parsed.fixed_cells, grid.fixed_cells);
        assert_eq!(parsed.root, grid.root);
    }
    #[test]
    fn test_lenient_metadata() {
        let parsed = Grid::parse_metadata_line(Some("root_row=3,root_column=5,"));
        assert_eq!(parsed.unwrap(), ivec2(5, 3));
        let parsed = Grid::parse_metadata_line(Some("root_row=3,root_column=5"));
        assert_eq!(parsed.unwrap(), ivec2(5, 3));
        let parsed = Grid::parse_metadata_line(Some("root_row = 3 , root_column = 5 "));
        assert_eq!(parsed.unwrap(), ivec2(5, 3));
        let parsed = Grid::parse_metadata_line(Some("root_row = 3 , root_column = 5 , "));
        assert_eq!(parsed.unwrap(), ivec2(5, 3));
    }
}

#[cfg(test)]
mod rails_tests {
    use super::*;

    #[test]
    fn test_recalculate_rails_to_top_left() {
        let mut grid = Grid::new(4, 4, ivec2(2, 2));
        *get_cell_mut(&mut grid, 1, 1) = true;
        grid.recalculate_rails();
        let crossing = grid.intersections.get(2, 2);
        assert_eq!(crossing, Intersection::new(Crossing::VerticalOnTop));
        assert_eq!(
            rails_to_string(&grid),
            r#"
•-•-•-•-•
| | | | |
•-o<o-•-•
| v ^ | |
•-o>">o-•
| | ^ v |
•-•-o<o-•
| | | | |
•-•-•-•-•
"#
            .trim_start_matches('\n')
        );
    }
    #[test]
    fn test_recalculate_rails_to_top_right() {
        let mut grid = Grid::new(4, 4, ivec2(1, 2));
        *get_cell_mut(&mut grid, 1, 2) = true;
        grid.recalculate_rails();
        let crossing = grid.intersections.get(2, 2);
        assert_eq!(crossing, Intersection::new(Crossing::HorizontalOnTop));
        assert_eq!(
            rails_to_string(&grid),
            r#"
•-•-•-•-•
| | | | |
•-•-o<o-•
| | v ^ |
•-o>=>o-•
| ^ v | |
•-o<o-•-•
| | | | |
•-•-•-•-•
"#
            .trim_start_matches('\n')
        );
    }
    #[test]
    fn test_recalculate_rails_to_bottom_left() {
        let mut grid = Grid::new(4, 4, ivec2(2, 1));
        *get_cell_mut(&mut grid, 2, 1) = true;
        grid.recalculate_rails();
        let crossing = grid.intersections.get(2, 2);
        assert_eq!(crossing, Intersection::new(Crossing::HorizontalOnTop));
        assert_eq!(
            rails_to_string(&grid),
            r#"
•-•-•-•-•
| | | | |
•-•-o>o-•
| | ^ v |
•-o<=<o-•
| v ^ | |
•-o>o-•-•
| | | | |
•-•-•-•-•
"#
            .trim_start_matches('\n')
        );
    }
    #[test]
    fn test_recalculate_rails_to_bottom_right() {
        let mut grid = Grid::new(4, 4, ivec2(1, 1));
        *get_cell_mut(&mut grid, 2, 2) = true;
        grid.recalculate_rails();
        let crossing = grid.intersections.get(2, 2);
        assert_eq!(crossing, Intersection::new(Crossing::VerticalOnTop));
        assert_eq!(
            rails_to_string(&grid),
            r#"
•-•-•-•-•
| | | | |
•-o>o-•-•
| ^ v | |
•-o<"<o-•
| | v ^ |
•-•-o>o-•
| | | | |
•-•-•-•-•
"#
            .trim_start_matches('\n')
        );
    }
    #[test]
    fn test_recalculate_rails_diagonal() {
        let mut grid = Grid::new(4, 5, ivec2(1, 1));
        *get_cell_mut(&mut grid, 2, 2) = true;
        *get_cell_mut(&mut grid, 2, 3) = true;
        grid.recalculate_rails();
    }
    #[test]
    fn test_recalculate_rails_below() {
        let mut grid = Grid::new(4, 3, ivec2(1, 1));
        *get_cell_mut(&mut grid, 2, 1) = true;
        grid.recalculate_rails();
    }
}
