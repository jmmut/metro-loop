use crate::levels::Levels;
use crate::AnyError;

pub struct LevelHistory {
    pub current: GameTrack,
    pub levels: Levels,
    pub solved: Vec<Vec<bool>>,
}

pub enum GameTrack {
    Campaign { section: i32, level: i32 },
    Procedural,
}

impl LevelHistory {
    pub fn new() -> Result<Self, AnyError> {
        let levels = Levels::get()?;
        let mut solved = Vec::new();
        for section in &levels.sections {
            let mut solved_in_section = Vec::new();
            solved_in_section.resize(section.levels.len(), false);
            solved.push(solved_in_section);
        }
        Ok(Self {
            current: GameTrack::Campaign {
                section: 0,
                level: 0,
            },
            levels,
            solved,
        })
    }
    pub fn next(&mut self) {
        match self.current {
            GameTrack::Campaign { section, level } => {
                for i_section in (section as usize)..self.levels.sections.len() {
                    for i_level in
                        (level as usize + 1)..self.levels.sections[i_section].levels.len()
                    {
                        if !self.solved[i_section][i_level] {
                            self.current = GameTrack::Campaign {
                                section: i_section as i32,
                                level: i_level as i32,
                            };
                            return;
                        }
                    }
                }
                self.current = GameTrack::Procedural;
            }
            GameTrack::Procedural => {}
        }
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
impl GameTrack {
    pub fn is_procedural(&self) -> bool {
        match self {
            GameTrack::Campaign { .. } => false,
            GameTrack::Procedural => true,
        }
    }
}
