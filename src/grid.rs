use crate::intersection::{inter_to_string, Crossing, Direction, Intersection, Intersections};
use crate::rails::Rails;
use juquad::widgets::anchor::{Horizontal, Vertical};
use macroquad::prelude::{ivec2, IVec2};
use std::fmt::{Display, Formatter};

pub type Cell = bool;
pub struct Grid {
    pub num_rows: i32,
    pub num_columns: i32,
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

        let mut grid = Self {
            num_rows,
            num_columns,
            cells,
            rails,
            root,
            intersections,
        };
        *get_mut(&mut grid, root.y, root.x) = true;
        grid
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
                let current = *get(&self, i_row, i_column);
                let above = *get(&self, i_row - 1, i_column);
                let left = *get(&self, i_row, i_column - 1);

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
                *self.rails.get_vert_mut(i_row, i_column) = direction;
            }
        }
        for i_row in 1..self.intersections.rows() - 1 {
            for i_column in 1..self.intersections.columns() - 1 {
                let above = self.rails.get_vert(i_row - 1, i_column);
                let below = self.rails.get_vert(i_row, i_column);
                let left = self.rails.get_horiz(i_row, i_column - 1);
                let right = self.rails.get_horiz(i_row, i_column);
                let right = Direction::from(right);
                let left = Direction::from(left).invert();
                let below = Direction::from(below);
                let above = Direction::from(above).invert();

                let cell_current = *get(&self, i_row, i_column);
                let cell_above = *get(&self, i_row - 1, i_column);
                let cell_left = *get(&self, i_row, i_column - 1);
                let cell_left_above = *get(&self, i_row - 1, i_column - 1);

                let crossing = if cell_current && cell_left_above && !cell_above && !cell_left {
                    Crossing::TopRightToBottomLeft
                } else if !cell_current && !cell_left_above && cell_above && cell_left {
                    Crossing::TopLeftToBottomRigt
                } else if cell_current || cell_left_above || cell_above || cell_left {
                    Crossing::Single
                } else {
                    Crossing::None
                };
                *self.intersections.get_mut(i_row, i_column) = Intersection {
                    // right,
                    // left,
                    // below,
                    // above,
                    crossing,
                }
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
                    grid_to_string(&self)
                );
            }
            rail_coord = if rail_is_horizontal {
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
                        } else if right == horizontal && right == Horizontal::Right  {
                            next_crossing
                        } else {
                            panic!()
                        }
                        // } else {
                        //     if below == Direction::Outwards {
                        //         rail_is_horizontal = !rail_is_horizontal;
                        //         next_crossing
                        //     } else if crossing.above == Direction::Outwards {
                        //         rail_is_horizontal = !rail_is_horizontal;
                        //         ivec2(next_crossing.x, next_crossing.y - 1)
                        //     } else if crossing.left == Direction::Outwards {
                        //         ivec2(next_crossing.x - 1, next_crossing.y)
                        //     } else if crossing.right == Direction::Outwards {
                        //         next_crossing
                        //     } else {
                        //         panic!()
                        //     }
                        // }
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
                        } else if below == vertical && below == Vertical::Bottom  {
                            next_crossing
                        } else {
                            panic!()
                        }
                        // } else {
                        //     if crossing.right == Direction::Outwards {
                        //         rail_is_horizontal = !rail_is_horizontal;
                        //         next_crossing
                        //     } else if crossing.left == Direction::Outwards {
                        //         rail_is_horizontal = !rail_is_horizontal;
                        //         ivec2(next_crossing.x - 1, next_crossing.y)
                        //     } else if crossing.above == Direction::Outwards {
                        //         ivec2(next_crossing.x, next_crossing.y - 1)
                        //     } else if crossing.below == Direction::Outwards {
                        //         next_crossing
                        //     } else {
                        //         panic!()
                        //     }
                        // }
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

pub fn in_range(grid: &Grid, row: i32, column: i32) -> bool {
    row > 0 && row < grid.rows() - 1 && column > 0 && column < grid.columns() - 1
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows() {
            for column in 0..self.columns() {
                let inter = self.intersections.get(row, column);
                write!(f, "{}", crate::intersection::crossing_to_char(inter))?;
                // write!(f, "{}", right_to_char(inter.right))?;
                let horiz = self.rails.get_horiz(row, column);
                write!(f, "{}", crate::intersection::horiz_to_char(horiz))?;
                // let inter = self.intersections.get(row, column+1);
                // write!(f, "{}", left_to_char(inter.left))?;
            }
            let inter = self.intersections.get(row, self.columns());
            write!(f, "{}", crate::intersection::crossing_to_char(inter))?;
            writeln!(f)?;
            // for column in 0..self.columns() +1 {
            //     let inter = self.intersections.get(row, column);
            //     write!(f, "{}   ", below_to_char(inter.below))?;
            // }
            // writeln!(f)?;
            for column in 0..self.columns(){
                let vert = self.rails.get_vert(row, column);
                write!(f, "{} ", crate::intersection::vert_to_char(vert))?;
            }
                let vert = self.rails.get_vert(row, self.columns());
                write!(f, "{}", crate::intersection::vert_to_char(vert))?;
            writeln!(f)?;
            // for column in 0..self.columns() +1 {
            //     let inter = self.intersections.get(row+1, column);
            //     write!(f, "{}   ", above_to_char(inter.above))?;
            // }
            // writeln!(f)?;
        }
        let row = self.rows();
        for column in 0..self.columns() {
            let inter = self.intersections.get(row, column);
            write!(f, "{}", crate::intersection::crossing_to_char(inter))?;
            // write!(f, "{}", right_to_char(inter.right))?;
            let horiz = self.rails.get_horiz(row, column);
            write!(f, "{}", crate::intersection::horiz_to_char(horiz))?;
            // let inter = self.intersections.get(row, column+1);
            // write!(f, "{}", left_to_char(inter.left))?;
        }
        let inter = self.intersections.get(row, self.columns());
        write!(f, "{}", crate::intersection::crossing_to_char(inter))?;
        writeln!(f)?;
        Ok(())
    }
}

pub fn grid_to_string(grid: &Grid) -> String {
    grid.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recalculate_rails_to_top_left() {
        let mut grid = Grid::new(4, 4, ivec2(2, 2));
        *get_mut(&mut grid, 1, 1) = true;
        grid.recalculate_rails();
        let crossing = grid.intersections.get(2, 2);
        assert_eq!(crossing, Intersection::new(Crossing::VerticalOnTop));
        assert_eq!(
            grid_to_string(&grid),
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
        *get_mut(&mut grid, 1, 2) = true;
        grid.recalculate_rails();
        let crossing = grid.intersections.get(2, 2);
        assert_eq!(crossing, Intersection::new(Crossing::HorizontalOnTop));
        assert_eq!(
            grid_to_string(&grid),
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
        *get_mut(&mut grid, 2, 1) = true;
        grid.recalculate_rails();
        let crossing = grid.intersections.get(2, 2);
        assert_eq!(crossing, Intersection::new(Crossing::HorizontalOnTop));
        assert_eq!(
            grid_to_string(&grid),
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
        *get_mut(&mut grid, 2, 2) = true;
        grid.recalculate_rails();
        let crossing = grid.intersections.get(2, 2);
        assert_eq!(crossing, Intersection::new(Crossing::VerticalOnTop));
        assert_eq!(
            grid_to_string(&grid),
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
        *get_mut(&mut grid, 2, 2) = true;
        *get_mut(&mut grid, 2, 3) = true;
        grid.recalculate_rails();
    }
    #[test]
    fn test_recalculate_rails_below() {
        let mut grid = Grid::new(4, 3, ivec2(1, 1));
        *get_mut(&mut grid, 1, 2) = true;
        grid.recalculate_rails();
    }
}
