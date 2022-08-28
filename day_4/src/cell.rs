#[derive(Debug, Clone)]
pub struct Cell {
    row: usize,
    col: usize,
    value: usize,
    marked: bool,
}

impl Cell {
    pub fn new(row: usize, col: usize, value: usize) -> Self {
        Self {
            row,
            col,
            value,
            marked: false,
        }
    }

    pub fn mark(&mut self) {
        self.marked = true;
    }

    pub fn get_value(&self) -> usize {
        self.value
    }

    pub fn get_row(&self) -> usize {
        self.row
    }

    pub fn get_col(&self) -> usize {
        self.col
    }

    pub fn get_marked(&self) -> bool {
        self.marked
    }
}
