use crate::logic::constraints::{Constraints, RailCoord};
use crate::logic::grid::{get, get_cell, Grid};
use crate::{generate_nested_vec, AnyError};
use juquad::widgets::anchor::{Horizontal, Vertical};
use macroquad::math::ivec2;
use std::fmt::{Display, Formatter};

// pub const raw_levels() -> Result<Levels, AnyError> {
pub const RAW_LEVELS: RawLevels = RawLevels {
    sections: &[RawSection {
        levels: &[r#". . . . . .
           
. x x x x .
           
. x . x x .
  " > >    
.=?^%=*vx .
  " < <    
. x x x x .
           
. . . . . .
"#],
    }],
};
pub struct Levels {
    sections: Vec<Section>,
}
pub struct RawLevels<'a> {
    sections: &'a [RawSection<'a>],
}

pub struct Section {
    levels: Vec<Level>,
}
pub struct RawSection<'a> {
    levels: &'a [&'a str],
}

pub struct Level {
    initial_grid: Grid,
    constraints: Constraints,
    solution: Grid,
}
impl Levels {
    pub fn get() -> Result<Levels, AnyError> {
        let mut sections = Vec::new();
        for raw_section in RAW_LEVELS.sections {
            let mut levels = Vec::new();
            for raw_level in raw_section.levels {
                let level = Level::from_str(raw_level)?;
                levels.push(level);
            }
            sections.push(Section { levels });
        }
        Ok(Levels { sections })
    }
}
impl Level {
    pub fn from_str(s: &str) -> Result<Level, AnyError> {
        let mut max_columns = 0;
        let mut cells = Vec::new();
        let mut fixed_cells = Vec::new();
        let mut solution_cells = Vec::new();
        let mut rails: Vec<RailCoord> = Vec::new();
        let mut cell_count = 0;
        let mut line_count = 0;
        let lines = s.lines();
        let mut root = None;
        enum Code {
            Cell {
                is_root: bool,
                cell: bool,
                fixed_cell: bool,
                solution: bool,
            },
            Constraint(RailCoord),
            NoRailConstraint,
        }
        for line in lines {
            line_count += 1;
            let mut cell_row = Vec::new();
            let mut fixed_cell_row = Vec::new();
            let mut solution_cell_row = Vec::new();
            let mut letter_count = 0;
            for letter in line.chars() {
                letter_count += 1;
                #[rustfmt::skip]
                let code = match letter {
                    '%' => Code::Cell{is_root: true, cell: true, fixed_cell: true, solution: true},
                    '@' => Code::Cell{is_root: false, cell: true, fixed_cell: true, solution: true},
                    '*' => Code::Cell{is_root: false, cell: false, fixed_cell: false, solution: true},
                    'O' => Code::Cell{is_root: false, cell: true, fixed_cell: false, solution: true},
                    'x' => Code::Cell{is_root: false, cell: false, fixed_cell: false, solution: false},
                    '?' => Code::Cell{is_root: false, cell: true, fixed_cell: false, solution: false},
                    '.' => Code::Cell{is_root: false, cell: false, fixed_cell: true, solution: false},
                    ' ' => Code::NoRailConstraint,
                    'v' => Code::Constraint(RailCoord::Vertical {row: (line_count-1)/2, column: (letter_count+1)/2, direction: Vertical::Bottom}),
                    '=' => Code::Constraint(RailCoord::Vertical {row: (line_count-1)/2, column: (letter_count+1)/2, direction: Vertical::Center}),
                    '^' => Code::Constraint(RailCoord::Vertical {row: (line_count-1)/2, column: (letter_count+1)/2, direction: Vertical::Top}),
                    '>' => Code::Constraint(RailCoord::Horizontal {row: (line_count+1)/2, column: (letter_count-1)/2, direction: Horizontal::Right}),
                    '"' => Code::Constraint(RailCoord::Horizontal {row: (line_count+1)/2, column: (letter_count-1)/2, direction: Horizontal::Center}),
                    '<' => Code::Constraint(RailCoord::Horizontal {row: (line_count+1)/2, column: (letter_count-1)/2, direction: Horizontal::Left}),
                    _ => {
                        return Err(format!(
                            "Wrong format for grid at (1-based) line {}, letter {}",
                            line_count, letter_count
                        )
                            .into())
                    }
                };
                match code {
                    Code::Cell {
                        is_root,
                        cell,
                        fixed_cell,
                        solution,
                    } => {
                        if is_root {
                            root = Some(ivec2((letter_count - 1) / 2, (line_count - 1) / 2));
                        }
                        if solution {
                            cell_count += 1;
                        }
                        cell_row.push(cell);
                        solution_cell_row.push(solution);
                        fixed_cell_row.push(fixed_cell);
                    }
                    Code::Constraint(rail_coord) => rails.push(rail_coord),
                    Code::NoRailConstraint => {}
                }
            }
            if cell_row.len() > max_columns {
                max_columns = cell_row.len();
            }
            if line_count % 2 == 1 {
                cells.push(cell_row);
                fixed_cells.push(fixed_cell_row);
                solution_cells.push(solution_cell_row);
            }
        }
        let Some(root) = root else {
            return Err("missing a root cell ('%')".into());
        };
        for row in &mut cells {
            row.resize(max_columns, false);
        }
        for row in &mut solution_cells {
            row.resize(max_columns, false);
        }
        for row in &mut fixed_cells {
            row.resize(max_columns, false);
        }
        let mut initial_grid = Grid::new_from_cells(
            cells.len() as i32,
            max_columns as i32,
            root,
            cells,
            fixed_cells.clone(),
        );
        initial_grid.recalculate_rails();
        let mut solution = Grid::new_from_cells(
            solution_cells.len() as i32,
            max_columns as i32,
            root,
            solution_cells,
            fixed_cells,
        );
        solution.recalculate_rails();
        let constraints = Constraints { rails, cell_count };
        Ok(Level {
            initial_grid,
            constraints,
            solution,
        })
    }
}
impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let columns = self.initial_grid.columns();
        let rows = self.initial_grid.rows();
        let mut lines = generate_nested_vec(rows as usize * 2 - 1, columns as usize * 2 - 1, ' ');
        for row in 0..rows {
            for column in 0..columns {
                let letter = if self.initial_grid.root == ivec2(column, row) {
                    '%'
                } else {
                    let cell = get_cell(&self.initial_grid, row, column);
                    let solution_cell = get_cell(&self.solution, row, column);
                    let fixed_cell = get(&self.solution.fixed_cells, row, column);
                    match (cell, fixed_cell, solution_cell) {
                        (true, true, true) => '@',
                        (true, false, true) => 'O',
                        (true, false, false) => '?',
                        (false, true, false) => '.',
                        (false, false, false) => 'x',
                        (false, false, true) => '*',
                        _ => panic!("logic error, should not reach here"),
                    }
                };
                lines[row as usize * 2][column as usize * 2] = letter;
            }
        }
        for constraint in &self.constraints.rails {
            match *constraint {
                RailCoord::Horizontal {
                    row,
                    column,
                    direction,
                } => {
                    lines[row as usize * 2 - 1][column as usize * 2] = match direction {
                        Horizontal::Left => '<',
                        Horizontal::Center => '"',
                        Horizontal::Right => '>',
                    };
                }
                RailCoord::Vertical {
                    row,
                    column,
                    direction,
                } => {
                    lines[row as usize * 2][column as usize * 2 - 1] = match direction {
                        Vertical::Top => '^',
                        Vertical::Center => '=',
                        Vertical::Bottom => 'v',
                    };
                }
            }
        }
        write!(
            f,
            "{}",
            lines
                .into_iter()
                .map(|mut chars| {
                    chars.push('\n');
                    chars.into_iter().collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("")
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::constraints::{compute_satisfaction, Satisfaction};

    #[test]
    fn basic_level() {
        let raw_level = RAW_LEVELS.sections[0].levels[0];
        let level = Level::from_str(raw_level).unwrap();
        assert_eq!(
            level.initial_grid.to_string(),
            r#"......
.    .
. .  .
.O%  .
.    .
......
"#
        );
        assert_eq!(
            level.solution.to_string(),
            r#"......
.    .
. .  .
. %O .
.    .
......
"#
        );
        assert_eq!(
            compute_satisfaction(&level.solution, &level.constraints).success(),
            true
        );
        assert_eq!(
            compute_satisfaction(&level.initial_grid, &level.constraints),
            Satisfaction {
                failing_rails: 8,
                cell_diff: 0,
                unconnected_loops: 1,
                unreachable_rails: 0,
            }
        );
    }

    #[test]
    fn roundtrip() {
        let raw_level = RAW_LEVELS.sections[0].levels[0];
        let level = Level::from_str(raw_level).unwrap();
        let serialized = level.to_string();
        assert_eq!(serialized, raw_level);
    }
}
