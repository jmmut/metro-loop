use crate::rails::Rails;
use crate::{NUM_COLUMNS, NUM_ROWS};
use juquad::widgets::anchor::{Horizontal, Vertical};
use macroquad::prelude::{ivec2, IVec2};
use crate::intersection::{Crossing, Direction, Intersection, Intersections};

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
        let mut rail_count = 0;
        for i_row in 1..NUM_ROWS {
            for i_column in 1..NUM_COLUMNS {
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
        for i_row in 1..self.intersections.rows() -1{
            for i_column in 1..self.intersections.columns() -1 {
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
                let cell_left_above = *get(&self, i_row -1, i_column - 1);

                let crossing = if cell_current && cell_left_above && !cell_above && !cell_left {
                    Crossing::TopRightToBottomLeft
                } else if !cell_current && !cell_left_above && cell_above && cell_left {
                    Crossing::TopLeftToBottomRigt
                } else if cell_current || cell_left_above || cell_above || cell_left {
                    Crossing::Full
                } else {
                    Crossing::None
                };
                *self.intersections.get_mut(i_row, i_column) = Intersection {
                    right, left, below, above, crossing,
                }
            }
        }
        let mut iterations = 0;
        let mut rail_coord = self.root;
        let mut rail_is_horizontal = true;
        let mut backwards = false;
        loop {
            iterations += 1;
            if iterations > 10000 {
                panic!("potential infinite loop in recalculate rails");
            }
            let row = rail_coord.y;
            let column = rail_coord.x;

            rail_coord =  if rail_is_horizontal {
                let horizontal = self.rails.get_horiz_mut(row, column);
                if backwards {
                    *horizontal = horizontal.opposite();
                }
                let next_crossing = match horizontal {
                    Horizontal::Left => rail_coord,
                    Horizontal::Center => panic!(),
                    Horizontal::Right => ivec2(column+1, row),
                };
                let crossing = self.intersections.get_mut(next_crossing.y, next_crossing.x);
                match crossing.crossing {
                    Crossing::None => panic!(),
                    Crossing::Full => {
                        if !backwards {
                            if crossing.below == Direction::Outwards {
                                rail_is_horizontal = !rail_is_horizontal;
                                next_crossing
                            } else if crossing.above == Direction::Outwards {
                                rail_is_horizontal = !rail_is_horizontal;
                                ivec2(next_crossing.x, next_crossing.y - 1)
                            } else if crossing.left == Direction::Outwards {
                                ivec2(next_crossing.x - 1, next_crossing.y)
                            } else if crossing.right == Direction::Outwards {
                                next_crossing
                            } else {
                                panic!()
                            }
                        } else {
                            if crossing.below == Direction::Inwards {
                                rail_is_horizontal = !rail_is_horizontal;
                                next_crossing
                            } else if crossing.above == Direction::Inwards {
                                rail_is_horizontal = !rail_is_horizontal;
                                ivec2(next_crossing.x, next_crossing.y - 1)
                            } else if crossing.left == Direction::Inwards {
                                ivec2(next_crossing.x - 1, next_crossing.y)
                            } else if crossing.right == Direction::Inwards {
                                next_crossing
                            } else {
                                panic!()
                            }
                        }
                    }
                    Crossing::TopLeftToBottomRigt => {
                        backwards = !backwards;
                        crossing.crossing = Crossing::HorizontalOnTop;
                        next_crossing
                    },
                    Crossing::TopRightToBottomLeft => {
                        backwards = !backwards;
                        crossing.crossing = Crossing::HorizontalOnTop;
                        next_crossing
                    }
                    Crossing::VerticalOnTop => {backwards=!backwards; next_crossing}
                    Crossing::HorizontalOnTop => {backwards=!backwards; next_crossing} // panic?
                }

            } else {
                let vertical = self.rails.get_vert_mut(row, column);
                if backwards {
                    *vertical = vertical.opposite();
                }
                let next_crossing = match vertical {
                    Vertical::Top => rail_coord,
                    Vertical::Center => panic!(),
                    Vertical::Bottom => ivec2(column, row+1),
                };
                let crossing = self.intersections.get_mut(next_crossing.y, next_crossing.x);
                match crossing.crossing {
                    Crossing::None => panic!(),
                    Crossing::Full => {
                        if !backwards {
                            if crossing.right == Direction::Outwards {
                                rail_is_horizontal = !rail_is_horizontal;
                                next_crossing
                            } else if crossing.left == Direction::Outwards {
                                rail_is_horizontal = !rail_is_horizontal;
                                ivec2(next_crossing.x - 1, next_crossing.y)
                            } else if crossing.above == Direction::Outwards {
                                ivec2(next_crossing.x, next_crossing.y - 1)
                            } else if crossing.below == Direction::Outwards {
                                next_crossing
                            } else {
                                panic!()
                            }
                        } else {
                            if crossing.right == Direction::Inwards {
                                rail_is_horizontal = !rail_is_horizontal;
                                next_crossing
                            } else if crossing.left == Direction::Inwards {
                                rail_is_horizontal = !rail_is_horizontal;
                                ivec2(next_crossing.x - 1, next_crossing.y)
                            } else if crossing.above == Direction::Inwards {
                                ivec2(next_crossing.x, next_crossing.y - 1)
                            } else if crossing.below == Direction::Inwards {
                                next_crossing
                            } else {
                                panic!()
                            }
                        }
                    }
                    Crossing::TopLeftToBottomRigt => {
                        backwards = !backwards;
                        crossing.crossing = Crossing::VerticalOnTop;
                        next_crossing
                    },
                    Crossing::TopRightToBottomLeft => {
                        backwards = !backwards;
                        crossing.crossing = Crossing::VerticalOnTop;
                        next_crossing
                    }
                    Crossing::VerticalOnTop => {backwards=!backwards; next_crossing} // panic?
                    Crossing::HorizontalOnTop =>  {backwards=!backwards; next_crossing}
                }
            };
            if rail_coord == self.root {
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

pub fn in_range(row: i32, column: i32) -> bool {
    row > 0 && row < NUM_ROWS - 1 && column > 0 && column < NUM_COLUMNS - 1
}
