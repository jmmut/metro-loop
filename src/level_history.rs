use crate::levels::{Level, Levels};
use crate::logic::constraints::{choose_constraints, count_unreachable_rails};
use crate::logic::grid::Grid;
use crate::scenes::play::generate_grid;
use crate::theme::Theme;
use crate::{AnyError, VISUALIZE};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct LevelHistory {
    pub levels: Levels,
}

#[derive(Debug)]
pub struct GameTrack {
    pub solved: Vec<Vec<bool>>,
    pub current: CurrentGame,
    // pub in_progress: Grid,
}

#[derive(Debug)]
pub enum CurrentGame {
    Campaign { section: i32, level: i32 },
    Procedural(Level),
}

impl GameTrack {
    pub fn new(section: i32, level: i32) -> Result<Self, AnyError> {
        let levels = Levels::get()?;
        let mut solved = Vec::new();
        for section in &levels.sections {
            let solved_in_section = vec![false; section.levels.len()];
            solved.push(solved_in_section);
        }
        Ok(Self {
            current: CurrentGame::Campaign { section, level },
            solved,
        })
    }
    pub fn get_current(&self, levels: &Levels) -> Level {
        match &self.current {
            CurrentGame::Campaign { section, level } => 
                levels
                .sections
                .get(*section as usize)
                .unwrap()
                .levels
                .get(*level as usize)
                .unwrap()
                .clone(),
            CurrentGame::Procedural(level) => level.clone(),
        }
    }
    pub async fn next(&mut self, theme: &Theme) -> &Self {
        match &mut self.current {
            CurrentGame::Campaign { section, mut level } => {
                level += 1;
                for i_section in (*section as usize)..theme.resources.levels.sections.len() {
                    for i_level in level as usize..theme.resources.levels.sections[i_section].levels.len() {
                        if !self.solved[i_section][i_level] {
                            self.current = CurrentGame::Campaign {
                                section: i_section as i32,
                                level: i_level as i32,
                            };
                            return self;
                        }
                    }
                    level = 0;
                }
                self.current = CurrentGame::Procedural(generate_procedural(VISUALIZE, theme).await);
            }
            CurrentGame::Procedural(level) => *level = generate_procedural(VISUALIZE, theme).await,
        }
        self
    }
    pub fn solved(&mut self) {
        match self.current {
            CurrentGame::Campaign { section, level } => {
                self.solved[section as usize][level as usize] = true;
            }
            CurrentGame::Procedural(_) => {}
        }
    }
}

pub async fn generate_procedural(visualize: bool, theme: &Theme) -> Level {
    let mut solution = generate_grid(visualize, theme).await;
    solution.recalculate_rails();
    while !count_unreachable_rails(&solution).success() {
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

impl CurrentGame {
    pub fn is_procedural(&self) -> bool {
        match self {
            CurrentGame::Campaign { .. } => false,
            CurrentGame::Procedural(_) => true,
        }
    }
}

impl Display for CurrentGame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CurrentGame::Campaign { section, level } => {
                write!(f, "Level: {}-{}", section, level)
            }
            CurrentGame::Procedural(_) => {
                write!(f, "Level: Random")
            }
        }
    }
}
