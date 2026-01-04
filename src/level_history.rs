use crate::levels::{Level, Levels};
use crate::logic::constraints::{choose_constraints, count_unreachable_rails};
use crate::logic::grid::{get, Grid};
use crate::scenes::play::generate_grid;
use crate::theme::Theme;
use crate::{AnyError, VISUALIZE};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct LevelHistory {
    pub levels: Levels,
}

pub type Solved = Vec<Vec<bool>>;

#[derive(Debug)]
pub struct GameTrack {
    pub solved: Solved,
    pub current: CurrentGame,
    pub in_progress: Grid,
    cached_level: Level,
}

#[derive(Debug)]
pub enum CurrentGame {
    Campaign { section: i32, level: i32 },
    Procedural,
}

impl GameTrack {
    pub fn new(section: i32, level: i32, levels: &Levels) -> Result<Self, AnyError> {
        let mut solved = Vec::new();
        for section in &levels.sections {
            let solved_in_section = vec![false; section.levels.len()];
            solved.push(solved_in_section);
        }
        let current = CurrentGame::Campaign { section, level };

        let cached_level = levels.get_level(section as usize, level as usize).clone();
        let in_progress = cached_level.initial_grid.clone();
        Ok(Self {
            current,
            solved,
            in_progress,
            cached_level,
        })
    }
    pub fn get_current_ids(&self) -> (i32, i32) {
        match self.current {
            CurrentGame::Campaign { section, level } => (section, level),
            CurrentGame::Procedural => (self.solved.len() as i32, 0),
        }
    }
    pub async fn get_next_unsolved_ids(&mut self, theme: &Theme) -> Option<(i32, i32)> {
        loop {
            let (section, level) = self.get_current_ids();
            if self.is_solved(section, level) {
                self.next(theme).await;
            } else {
                return Some((section, level));
            }
        }
    }
    pub fn get_current(&self) -> &Level {
        &self.cached_level
    }
    pub async fn next(&mut self, theme: &Theme) -> &Self {
        match &mut self.current {
            CurrentGame::Campaign { section, mut level } => {
                level += 1;
                for i_section in (*section as usize)..theme.resources.levels.sections.len() {
                    for i_level in
                        level as usize..theme.resources.levels.sections[i_section].levels.len()
                    {
                        if !self.solved[i_section][i_level] {
                            self.current = CurrentGame::Campaign {
                                section: i_section as i32,
                                level: i_level as i32,
                            };
                            self.cached_level =
                                theme.resources.levels.get_level(i_section, i_level).clone();
                            self.in_progress = self.cached_level.initial_grid.clone();
                            return self;
                        }
                    }
                    level = 0;
                }
                self.cached_level = generate_procedural(VISUALIZE, theme).await;
                self.in_progress = self.cached_level.initial_grid.clone();
                self.current = CurrentGame::Procedural;
            }
            CurrentGame::Procedural => {
                self.cached_level = generate_procedural(VISUALIZE, theme).await;
                self.in_progress = self.cached_level.initial_grid.clone();
            }
        }
        self
    }
    pub fn is_solved(&self, section: i32, level: i32) -> bool {
        if self.is_random_index(section, level) {
            false
        } else {
            *get(&self.solved, section, level)
        }
    }
    pub fn is_random_index(&self, section: i32, level: i32) -> bool {
        section == self.solved.len() as i32 && level == 0
    }
    pub fn solved(&mut self) {
        match self.current {
            CurrentGame::Campaign { section, level } => {
                self.solved[section as usize][level as usize] = true;
            }
            CurrentGame::Procedural => {}
        }
    }
    pub async fn select(
        &mut self,
        section: i32,
        level: i32,
        levels: &Levels,
        theme: &Theme,
    ) -> bool {
        if let Some(level_copy) = levels.maybe_get_level(section, level).cloned() {
            self.current = CurrentGame::Campaign { section, level };
            self.cached_level = level_copy;
            self.in_progress = self.cached_level.initial_grid.clone();
            true
        } else if self.is_random_index(section, level) {
            if let CurrentGame::Procedural = self.current {
            } else {
                self.current = CurrentGame::Procedural;
                self.cached_level = generate_procedural(VISUALIZE, theme).await;
                self.in_progress = self.cached_level.initial_grid.clone();
            }
            true
        } else {
            false
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
            CurrentGame::Procedural => true,
        }
    }
}

impl Display for CurrentGame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CurrentGame::Campaign { section, level } => {
                write!(f, "Level: {}-{}", section, level)
            }
            CurrentGame::Procedural => {
                write!(f, "Level: Random")
            }
        }
    }
}
