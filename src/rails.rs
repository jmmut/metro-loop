use crate::generate_nested_vec;
use juquad::widgets::anchor::{Horizontal, Vertical};

pub struct Rails {
    pub horizontal: Vec<Vec<Horizontal>>,
    pub vertical: Vec<Vec<Vertical>>,
    pub reachable_horizontal: Vec<Vec<bool>>,
    pub reachable_vertical: Vec<Vec<bool>>,
}

impl Rails {
    pub fn new(num_rows: i32, num_columns: i32) -> Self {
        let num_rows = num_rows as usize;
        let num_rows_1 = num_rows + 1;
        let num_columns = num_columns as usize;
        let num_columns_1 = num_columns + 1;

        let horizontal = generate_nested_vec(num_rows_1, num_columns, Horizontal::Center);
        let vertical = generate_nested_vec(num_rows, num_columns_1, Vertical::Center);

        let reachable_horizontal = generate_nested_vec(num_rows_1, num_columns, false);
        let reachable_vertical = generate_nested_vec(num_rows, num_columns_1, false);

        Self {
            horizontal,
            vertical,
            reachable_horizontal,
            reachable_vertical,
        }
    }
    pub fn horiz_rows(&self) -> i32 {
        self.horizontal.len() as i32
    }
    pub fn horiz_columns(&self) -> i32 {
        self.horizontal.get(0).unwrap().len() as i32
    }
    pub fn vert_rows(&self) -> i32 {
        self.vertical.len() as i32
    }
    pub fn vert_columns(&self) -> i32 {
        self.vertical.get(0).unwrap().len() as i32
    }
    pub fn get_horiz(&self, row: i32, column: i32) -> Horizontal {
        *self
            .horizontal
            .get(row as usize)
            .unwrap()
            .get(column as usize)
            .unwrap()
    }
    pub fn get_horiz_mut(&mut self, row: i32, column: i32) -> &mut Horizontal {
        self.horizontal
            .get_mut(row as usize)
            .unwrap()
            .get_mut(column as usize)
            .unwrap()
    }
    pub fn get_vert(&self, row: i32, column: i32) -> Vertical {
        *self
            .vertical
            .get(row as usize)
            .unwrap()
            .get(column as usize)
            .unwrap()
    }
    pub fn get_vert_mut(&mut self, row: i32, column: i32) -> &mut Vertical {
        self.vertical
            .get_mut(row as usize)
            .unwrap()
            .get_mut(column as usize)
            .unwrap()
    }
    pub fn get_reach_horiz(&self, row: i32, column: i32) -> bool {
        *self
            .reachable_horizontal
            .get(row as usize)
            .unwrap()
            .get(column as usize)
            .unwrap()
    }
    pub fn get_reach_horiz_mut(&mut self, row: i32, column: i32) -> &mut bool {
        self.reachable_horizontal
            .get_mut(row as usize)
            .unwrap()
            .get_mut(column as usize)
            .unwrap()
    }
    pub fn get_reach_vert(&self, row: i32, column: i32) -> bool {
        *self
            .reachable_vertical
            .get(row as usize)
            .unwrap()
            .get(column as usize)
            .unwrap()
    }
    pub fn get_reach_vert_mut(&mut self, row: i32, column: i32) -> &mut bool {
        self.reachable_vertical
            .get_mut(row as usize)
            .unwrap()
            .get_mut(column as usize)
            .unwrap()
    }
}
