use juquad::widgets::anchor::{Horizontal, Vertical};

pub struct Rails {
    pub horizontal: Vec<Vec<Horizontal>>,
    pub vertical: Vec<Vec<Vertical>>,
}

impl Rails {
    pub fn new(num_rows: i32, num_columns: i32) -> Self {
        let mut row = Vec::new();
        row.resize(num_columns as usize, Horizontal::Center);
        let mut horizontal = Vec::new();
        horizontal.resize(num_rows as usize + 1, row);

        let mut row = Vec::new();
        row.resize(num_columns as usize + 1, Vertical::Center);
        let mut vertical = Vec::new();
        vertical.resize(num_rows as usize, row);

        Self {
            horizontal,
            vertical,
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
}
