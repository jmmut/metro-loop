use crate::levels::{Level, Levels};
use crate::logic::constraints::{choose_constraints, count_unreachable_rails};
use crate::logic::grid::Grid;
use crate::scenes::play::generate_grid;
use crate::theme::Theme;
use crate::AnyError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct LevelHistory {
    pub current: GameTrack,
    pub levels: Levels,
    pub solved: Vec<Vec<bool>>,
}

#[derive(Debug)]
pub enum GameTrack {
    Campaign { section: i32, level: i32 },
    Procedural,
}

impl LevelHistory {
    pub fn new(section: i32, level: i32) -> Result<Self, AnyError> {
        let levels = Levels::get()?;
        let mut solved = Vec::new();
        for section in &levels.sections {
            let solved_in_section = vec![false; section.levels.len()];
            solved.push(solved_in_section);
        }
        Ok(Self {
            current: GameTrack::Campaign { section, level },
            levels,
            solved,
        })
    }
    pub fn get_current(&self) -> Option<Level> {
        match self.current {
            GameTrack::Campaign { section, level } => Some(
                self.levels
                    .sections
                    .get(section as usize)?
                    .levels
                    .get(level as usize)?
                    .clone(),
            ),
            GameTrack::Procedural => None,
        }
    }
    pub fn next(&mut self) -> &Self {
        match self.current {
            GameTrack::Campaign { section, mut level } => {
                level += 1;
                for i_section in (section as usize)..self.levels.sections.len() {
                    for i_level in level as usize..self.levels.sections[i_section].levels.len() {
                        if !self.solved[i_section][i_level] {
                            self.current = GameTrack::Campaign {
                                section: i_section as i32,
                                level: i_level as i32,
                            };
                            return self;
                        }
                    }
                    level = 0;
                }
                self.current = GameTrack::Procedural;
            }
            GameTrack::Procedural => {}
        }
        self
    }
    pub fn solved(&mut self) {
        match self.current {
            GameTrack::Campaign { section, level } => {
                self.solved[section as usize][level as usize] = true;
            }
            GameTrack::Procedural => {}
        }
    }
}

pub async fn generate_procedural(visualize: bool, theme: &Theme) -> Level {
    let mut solution = generate_grid(visualize, theme).await;
    solution.recalculate_rails();
    while count_unreachable_rails(&solution) > 0 {
        solution = generate_grid(visualize, theme).await;
        solution.recalculate_rails();
    }
    // println!("tried {} iterations", i);
    let mut grid = Grid::new(solution.rows(), solution.columns(), solution.root);
    grid.recalculate_rails();
    let constraints = choose_constraints(&solution);
    Level {
        initial_grid: grid,
        constraints,
        solution,
    }
}

impl GameTrack {
    pub fn is_procedural(&self) -> bool {
        match self {
            GameTrack::Campaign { .. } => false,
            GameTrack::Procedural => true,
        }
    }
}

impl Display for GameTrack {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GameTrack::Campaign { section, level } => {
                write!(f, "Level: {}-{}", section, level)
            }
            GameTrack::Procedural => {
                write!(f, "Level: Random")
            }
        }
    }
}
