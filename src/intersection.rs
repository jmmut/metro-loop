use juquad::widgets::anchor::{Horizontal, Vertical};
use crate::generate_nested_vec;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Inwards,
    Outwards,
    Absent,
}
impl Direction {
    pub fn invert(self) -> Self {
        match self {
            Direction::Inwards => Direction::Outwards,
            Direction::Outwards => Direction::Inwards,
            Direction::Absent => Direction::Absent,
        }
    }
}

impl From<Vertical> for Direction {
    fn from(value: Vertical) -> Self {
        match value {
            Vertical::Top => Direction::Inwards,
            Vertical::Center => Direction::Absent,
            Vertical::Bottom => Direction::Outwards,
        }
    }
}
impl From<Horizontal> for Direction {
    fn from(value: Horizontal) -> Self {
        match value {
            Horizontal::Left => Direction::Inwards,
            Horizontal::Center => Direction::Absent,
            Horizontal::Right => Direction::Outwards,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Crossing {
    None,
    Full,
    TopLeftToBottomRigt,
    TopRightToBottomLeft,
    VerticalOnTop,
    HorizontalOnTop,
}
#[derive(Copy, Clone)]
pub struct Intersection {
    pub right: Direction,
    pub left: Direction,
    pub below: Direction,
    pub above: Direction,
    pub crossing: Crossing,
}

impl Default for Intersection {
    fn default() -> Self {
        Self {
            left: Direction::Absent,
            right: Direction::Absent,
            above: Direction::Absent,
            below: Direction::Absent,
            crossing: Crossing::None,
        }
    }
}
pub struct Intersections {
    inner: Vec<Vec<Intersection>>,
}

impl Intersections {
    pub fn new(num_rows: i32, num_columns: i32) -> Self {
        let inner = generate_nested_vec(num_rows as usize + 1, num_columns as usize + 1, Intersection::default());
        Self {
            inner,
        }
    }


    pub fn rows(&self) -> i32 {
        self.inner.len() as i32
    }
    pub fn columns(&self) -> i32 {
        self.inner.get(0).unwrap().len() as i32
    }
    pub fn get(&self, row: i32, column: i32) -> Intersection {
        *self
            .inner
            .get(row as usize)
            .unwrap()
            .get(column as usize)
            .unwrap()
    }
    pub fn get_mut(&mut self, row: i32, column: i32) -> &mut Intersection {
        self.inner
            .get_mut(row as usize)
            .unwrap()
            .get_mut(column as usize)
            .unwrap()
    }
}
