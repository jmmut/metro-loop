use crate::logic::constraints::{Constraints, RailCoord};
use crate::logic::grid::{Grid, get, get_cell};
use crate::{AnyError, generate_nested_vec};
use juquad::widgets::anchor::{Horizontal, Vertical};
use macroquad::math::ivec2;
use std::fmt::{Display, Formatter};

// pub const raw_levels() -> Result<Levels, AnyError> {
pub const RAW_LEVELS: RawLevels = RawLevels {
    sections: &[
        RawSection {
            levels: &[
//                 (
//                     "If it seems obvious it's because it is",
//                 r#". . . . .
//          
// . x x x .
//          
// . x . x .
//     > >  
// . x^% *v.
//     < <  
// . x x x .
//          
// . x x x .
//          
// . . . . .
// "#),
                (
                    "Don't overthink it",
                r#". . . . .
         
. x x x .
         
. x . x .
    >    
. x % x .
         
. x * x .
    <    
. x x x .
         
. . . . .
"#),
                (
                    "Your first diagonal",
                r#". . . . .
         
. x x x .
         
. *^. x .
  > >    
. x^% x .
         
. x x x . 
         
. x x x .
         
. . . . .
"#),
                (
                    "Yagami's demise",
                r#". . . . .
         
. x x x .
         
. x . x .
    >    
. x % x .
         
. x * *. 
      <  
. x x x .
         
. . . . .
"#),
                (
                    "Notice the reversal",
                r#". . . . .
         
. x x x .
         
. x . x .
    >    
. x % x .
         
. x ? O. 
      >  
. x x x .
         
. . . . .
"#),
                (
                    "Your first fence",
                r#".-.-.-.-.
---------
.-x-x-x-.
---------
.-x-.-x-.
---->----
.-x-%-x-.
----"----
.-x-x-x-.
---------
.-x-x-x-.
---------
.-.-.-.-.
"#),
                (
                    "Fences are fine inside or outside",
                r#".-.-.-.-.
---------
.-*=*-x-.
--"------
.-*-.-x-.
--"->-"--
.-O=%-x-.
---------
.-x-x-x-.
---------
.-x-x-x-.
---------
.-.-.-.-.
"#),
                (
                    "Think in loops",
                r#".-.-.-.-.
------<--
.-*-*-*-.
----<----
.v*-.-*-.
---->----
.=x-%-*-.
---------
.=x-x-x-.
----"----
.-x=x=x-.
------"--
.-.-.-.-.
"#),
            ],
        },
        RawSection {
            levels: &[
                (
                    "Your first real puzzle",
                r#".-.-.-.-.-.-.
-------------
.=xv*-x-x-x-.
----------"--
.v*=*-.-x-x-.
---->->-"----
.-x-x^%-x-x-.
-------------
.-x-x-*=*-x-.
----"--------
.-x-x-*-*-x-.
------<-<----
.-.-.-.-.-.-.

"#),
                (
                    "Let's change some constraints",
                r#".-.-.-.-.-.-.
----<--------
.=xvO-x-x-x-.
----------"--
.vO=O-.-*-x-.
---->->-"----
.v*^x^%-*vx-.
------>------
.-x-*-?=?-*-.
----"--------
.-x-*-O-O=*-.
----<-<-<----
.-.-.-.-.-.-.
"#),
                (
                    "Am I too annoying?",
                r#".-.-.-.-.-.-.
----<--------
.=xvO-*-*-x-.
----------"--
.vO=O-.-O-x-.
---->->-"----
.vO^xv%-O^x-.
------>------
.-x-O-x=x-O-.
----"--------
.-x-O-O-O=O-.
----<-<-<----
.-.-.-.-.-.-.
"#),
            ],
        },
        RawSection {
            levels: &[
                (
                    "TODO 1",
                r#".-.-.-.-.-.-.-.-.
--"------->------
.-x-x-x-x-*-*-*-.
----"------->----
.-x-x-*-*-x=x-*-.
--------<-<-<----
.-x-*=*-.-*-*=*-.
-->---"->---"----
.-*-x-*-%=*-*-x-.
--<-<-----<------
.=x^*-x^*-x^*-x-.
--"-<---<--------
.-x-x-x-x=x=xv*-.
----"------------
.-.-.-.-.-.-.-.-.
"#),
                (
                    "TODO 2",
                r#".-.-.-.-.-.-.-.-.
-----------------
.-x-x-x-*-*-x-x=.
-----------------
.v*-*-*-x-*=*-*-.
-----------------
.-xv*-x-.-*^xv*-.
-------->----->--
.-xv*vx-%-x^*-x-.
---------->------
.-x-x-*-x-*-*-x-.
--------<-<------
.-x-x-*-*-x-x-x-.
-----------------
.-.-.-.-.-.-.-.-.
"#),
                (
                    "TODO 3",
                r#".-.-.-.-.-.-.-.-.
--------"--------
.v*=*-*-xv*=*-*^.
------>----------
.-*-*^x-*-x-*-*^.
------"-----<----
.-x=x-x-.v*-x-*-.
-------->--->----
.-x=x-x^%-x-*-*-.
------"-----<-"--
.-x-x=x=x-*vx-*-.
----"---------"--
.-x-x-x-x-xv*=*^.
----"--------->--
.-.-.-.-.-.-.-.-.
"#),
            ],
        },
        RawSection {
            levels: &[
                //                 r#".-.-.-.-.-.-.-.-.
                // -----------------
                // .-x-x-x-x-x-x-x-.
                // -----------------
                // .-x-x-x-x-x-x-x-.
                // -----------------
                // .-x-x-x-.-x-x-x-.
                // -------->--------
                // .-x-x-x-%-x-x-x-.
                // -----------------
                // .-x-x-x-x-x-x-x-.
                // -----------------
                // .-x-x-x-x-x-x-x-.
                // -----------------
                // .-.-.-.-.-.-.-.-.
                // "#,
                (
                    "TODO 4",
                r#".-.-.-.-.-.-.-.-.-.-.
----"->----->----->--
.-x-x-*-*-*-*vx^*-*v.
------"---------"-<--
.-x-x-*-*-*-x-*=*-x-.
--------<-"-<---"----
.=x-x-*-x^*-*-x^*-x=.
--"-<--------->------
.-x-*-x-*-.^*=*=*vx=.
---------->-------"--
.-x-x-*vx^%^x-*-*-x-.
------<-------"---<--
.-x-*^x=x-x-*-*-xv*-.
--------------<------
.-*-x-x-x-*-x=x-*-*-.
------------>----->--
.-*-*vx=x=x^*=*-x=x=.
----------------"----
.-.-.-.-.-.-.-.-.-.-.
"#),
                (
                    "TODO 5",
                r#".-.-.-.-.-.-.-.-.-.-.
----"-------------<--
.-*-x-*=*-*-*^x-O=O-.
------"---<----------
.-O-*-*-*vx^*-*-*-*-.
--"---------------"--
.-O^xvO-*-*-x-*-x-*-.
------<--->----------
.-O-Ovx-O^.-x=x-*-O^.
--"--->--->----------
.-O=O-O-x^%-*-x=x=x-.
-->-----<-<----------
.-x-Ovx-O^x=x-x=x-x-.
----"->-------"---"--
.-x-O=O^x-x=x-x=x=x-.
------------"--------
.-x-*-x-x-x=x=x-x=x-.
------------"-"------
.-.-.-.-.-.-.-.-.-.-.
"#),
                (
                    "Interesting corners",
                    r#".-.-.-.-.-.-.-.-.-.-.
--"--------------->--
.-xvO-x=x^O=Ovx-O=O-.
---->->--------------
.=x=x-O-O-O=O=O-?-*-.
--"-----------"------
.-x=x^O-x-x-*-O=O-O-.
--"------------------
.-x-O-x=x=.-O-x^O-Ov.
---------->--->---"--
.^O-x-x-x-%-x-O-?-Ov.
----------<-----<----
.-O-xvO-x=x-O-*=*-x=.
--------<------------
.-x-O-x-O^x-Ovx^O=O-.
--"-<-"-----<--------
.-x-x-x-?-Ovx-?-x^O-.
----------------"----
.-.-.-.-.-.-.-.-.-.-.
"#,
                ),
            ],
        },
    ],
};
#[derive(Debug)]
pub struct Levels {
    pub sections: Vec<Section>,
}
pub struct RawLevels<'a> {
    sections: &'a [RawSection<'a>],
}

#[derive(Debug)]
pub struct Section {
    pub levels: Vec<Level>,
}
pub struct RawSection<'a> {
    levels: &'a [(&'a str, &'a str)],
}

#[derive(Clone, Debug)]
pub struct Level {
    pub initial_grid: Grid,
    pub constraints: Constraints,
    pub solution: Grid,
    pub subtitle: String,
}
impl Levels {
    pub fn get() -> Result<Levels, AnyError> {
        let mut sections = Vec::new();
        for raw_section in RAW_LEVELS.sections {
            let mut levels = Vec::new();
            for (subtitle, raw_level) in raw_section.levels {
                let level = Level::from_str_and_subtitle(raw_level, subtitle)?;
                levels.push(level);
            }
            sections.push(Section { levels });
        }
        Ok(Levels { sections })
    }
    pub fn get_level(&self, section: usize, level: usize) -> &Level {
        self.sections
            .get(section)
            .unwrap()
            .levels
            .get(level)
            .unwrap()
    }
    pub fn maybe_get_level(&self, section: i32, level: i32) -> Option<&Level> {
        self.sections
            .get(section as usize)?
            .levels
            .get(level as usize)
    }
}
impl Level {
    pub fn from_str(s: &str) -> Result<Level, AnyError> {
        Level::from_str_and_subtitle(s, "")
    }
    pub fn from_str_and_subtitle(s: &str, subtitle: &str) -> Result<Level, AnyError> {
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
                    ' ' | '-' => Code::NoRailConstraint,
                    'v' => Code::Constraint(RailCoord::Vertical {row: (line_count-1)/2, column: (letter_count+1)/2, sense: Vertical::Bottom}),
                    '=' => Code::Constraint(RailCoord::Vertical {row: (line_count-1)/2, column: (letter_count+1)/2, sense: Vertical::Center}),
                    '^' => Code::Constraint(RailCoord::Vertical {row: (line_count-1)/2, column: (letter_count+1)/2, sense: Vertical::Top}),
                    '>' => Code::Constraint(RailCoord::Horizontal {row: (line_count+1)/2, column: (letter_count-1)/2, sense: Horizontal::Right}),
                    '"' => Code::Constraint(RailCoord::Horizontal {row: (line_count+1)/2, column: (letter_count-1)/2, sense: Horizontal::Center}),
                    '<' => Code::Constraint(RailCoord::Horizontal {row: (line_count+1)/2, column: (letter_count-1)/2, sense: Horizontal::Left}),
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
            subtitle: subtitle.to_string(),
        })
    }
}
impl Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let columns = self.initial_grid.columns();
        let rows = self.initial_grid.rows();
        let mut lines = generate_nested_vec(rows as usize * 2 - 1, columns as usize * 2 - 1, '-');
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
                    sense: direction,
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
                    sense: direction,
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
    use crate::logic::constraints::{Goal, Satisfaction, compute_satisfaction};

    const RAW_LEVEL: &str = r#".-.-.-.-.-.
-----------
.-x-x-x-x-.
-----------
.-x-.-x-x-.
--"->->----
.=?^%=*vx-.
--"-<-<----
.-x-x-x-x-.
-----------
.-.-.-.-.-.
"#;

    #[test]
    fn basic_level() {
        let level = Level::from_str(RAW_LEVEL).unwrap();
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
                stations: Goal::new(2, 6),
                bridges: Goal::new(0, 4),
                cell_count: Goal::new(2, 2),
                reachable: Goal::new(6, 6),
            }
        );
    }

    #[test]
    fn roundtrip() {
        let level = Level::from_str(RAW_LEVEL).unwrap();
        let serialized = level.to_string();
        assert_eq!(serialized, RAW_LEVEL);
    }
}
