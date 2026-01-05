use macroquad::math::{vec2, IVec2, Vec2};
use std::ops::{Add, Sub};

#[derive(Copy, Clone)]
pub struct CellSpot {
    corner: bool,
    rounded: Coord,
    floored: Coord,
}

impl CellSpot {
    pub fn new(pos: Coord) -> Self {
        let pos: Vec2 = pos.into();
        let sum = pos.x.fract() + pos.y.fract();
        let diff = (pos.x.fract() - pos.y.fract()).abs();
        let corner = sum < 0.5 || sum > 1.5 || diff > 0.5;
        let rounded = pos.round().into();
        let floored = pos.floor().into();
        CellSpot {
            corner,
            rounded,
            floored,
        }
    }
    pub fn is_corner(&self) -> bool {
        self.corner
    }
    pub fn corner(&self) -> Coord {
        self.floored + self.quadrant()
    }

    pub fn quadrant(&self) -> Coord {
        self.rounded - self.floored
    }
    pub fn diff_rounded(&self, other: CellSpot) -> Coord {
        self.rounded - other.rounded
    }
    pub fn diff_floored(&self, other: CellSpot) -> Coord {
        self.floored - other.floored
    }
    pub fn floored(&self) -> Coord {
        self.floored
    }
    pub fn rounded(&self) -> Coord {
        self.rounded
    }
}

pub fn manhattan_distance(vec: IVec2) -> i32 {
    vec.x + vec.y
}

pub fn to_ivec(v: Vec2) -> IVec2 {
    let Vec2 { x, y } = v.floor();
    IVec2::new(x as i32, y as i32)
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Coord {
    pub row: f32,
    pub column: f32,
}
impl Coord {
    pub fn new_f(row: f32, column: f32) -> Self {
        Self { row, column }
    }
    pub fn new_i(row: i32, column: i32) -> Self {
        Self::new_f(row as f32, column as f32)
    }
    pub fn row(&self) -> i32 {
        self.row.floor() as i32
    }
    pub fn column(&self) -> i32 {
        self.column.floor() as i32
    }
    pub fn row_f(&self) -> f32 {
        self.row
    }
    pub fn column_f(&self) -> f32 {
        self.column
    }
    pub fn abs(self) -> Coord {
        Coord {
            row: self.row.abs(),
            column: self.column.abs(),
        }
    }
    pub fn floor(self) -> Coord {
        Coord {
            row: self.row.floor(),
            column: self.column.floor(),
        }
    }
    pub fn into<T: From<Coord>>(self) -> T {
        Into::<T>::into(self)
    }
}
impl From<IVec2> for Coord {
    fn from(value: IVec2) -> Self {
        Coord {
            row: value.y as f32,
            column: value.x as f32,
        }
    }
}
impl From<Coord> for IVec2 {
    fn from(value: Coord) -> Self {
        to_ivec(value.into())
    }
}
impl From<Vec2> for Coord {
    fn from(value: Vec2) -> Self {
        Coord {
            row: value.y,
            column: value.x,
        }
    }
}
impl From<Coord> for Vec2 {
    fn from(value: Coord) -> Self {
        vec2(value.column, value.row)
    }
}
impl Add<Coord> for Coord {
    type Output = Coord;
    fn add(self, other: Coord) -> Self::Output {
        Coord {
            row: self.row + other.row,
            column: self.column + other.column,
        }
    }
}
impl Sub<Coord> for Coord {
    type Output = Coord;
    fn sub(self, other: Coord) -> Self::Output {
        Coord {
            row: self.row - other.row,
            column: self.column - other.column,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_00_corner() {
        let pos = Coord::new_f(1.2, 1.25);
        let spot = CellSpot::new(pos);
        assert_eq!(spot.is_corner(), true);
        assert_eq!(spot.quadrant(), Coord::new_i(0, 0))
    }
    #[test]
    fn test_00_center() {
        let pos = Coord::new_f(1.3, 1.25);
        let spot = CellSpot::new(pos);
        assert_eq!(spot.is_corner(), false);
        assert_eq!(spot.quadrant(), Coord::new_i(0, 0))
    }
    #[test]
    fn test_01_corner() {
        let pos = Coord::new_f(1.2, 1.9);
        let spot = CellSpot::new(pos);
        assert_eq!(spot.is_corner(), true);
        assert_eq!(spot.quadrant(), Coord::new_i(0, 1))
    }
    #[test]
    fn test_01_center() {
        let pos = Coord::new_f(1.2, 1.55);
        let spot = CellSpot::new(pos);
        assert_eq!(spot.is_corner(), false);
        assert_eq!(spot.quadrant(), Coord::new_i(0, 1))
    }
    #[test]
    fn test_10_corner() {
        let pos = Coord::new_f(1.8, 1.25);
        let spot = CellSpot::new(pos);
        assert_eq!(spot.is_corner(), true);
        assert_eq!(spot.quadrant(), Coord::new_i(1, 0))
    }
    #[test]
    fn test_10_center() {
        let pos = Coord::new_f(1.6, 1.25);
        let spot = CellSpot::new(pos);
        assert_eq!(spot.is_corner(), false);
        assert_eq!(spot.quadrant(), Coord::new_i(1, 0))
    }
    #[test]
    fn test_11_corner() {
        let pos = Coord::new_f(1.8, 1.9);
        let spot = CellSpot::new(pos);
        assert_eq!(spot.is_corner(), true);
        assert_eq!(spot.quadrant(), Coord::new_i(1, 1))
    }
    #[test]
    fn test_11_center() {
        let pos = Coord::new_f(1.6, 1.55);
        let spot = CellSpot::new(pos);
        assert_eq!(spot.is_corner(), false);
        assert_eq!(spot.quadrant(), Coord::new_i(1, 1))
    }
}
