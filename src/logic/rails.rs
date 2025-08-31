use crate::generate_nested_vec;

#[derive(Clone)]
pub struct Rails<H, V> {
    pub horizontal: Vec<Vec<H>>,
    pub vertical: Vec<Vec<V>>,
}

impl<H: Clone, V: Clone> Rails<H, V> {
    pub fn new(
        num_rows: i32,
        num_columns: i32,
        horizontal_default: H,
        vertical_default: V,
    ) -> Self {
        let num_rows = num_rows as usize;
        let num_rows_1 = num_rows + 1;
        let num_columns = num_columns as usize;
        let num_columns_1 = num_columns + 1;

        let horizontal = generate_nested_vec(num_rows_1, num_columns, horizontal_default);
        let vertical = generate_nested_vec(num_rows, num_columns_1, vertical_default);

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
    pub fn get_horiz(&self, row: i32, column: i32) -> H {
        self.horizontal
            .get(row as usize)
            .unwrap()
            .get(column as usize)
            .unwrap()
            .clone()
    }
    pub fn get_horiz_mut(&mut self, row: i32, column: i32) -> &mut H {
        self.horizontal
            .get_mut(row as usize)
            .unwrap()
            .get_mut(column as usize)
            .unwrap()
    }
    pub fn get_vert(&self, row: i32, column: i32) -> V {
        self.vertical
            .get(row as usize)
            .unwrap()
            .get(column as usize)
            .unwrap()
            .clone()
    }
    pub fn get_vert_mut(&mut self, row: i32, column: i32) -> &mut V {
        self.vertical
            .get_mut(row as usize)
            .unwrap()
            .get_mut(column as usize)
            .unwrap()
    }
}
