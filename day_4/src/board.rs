use crate::cell::Cell;

#[derive(Debug, Clone)]
pub struct Board {
    board_size: usize,
    cells: Vec<Cell>,
}

impl Board {
    pub fn new(board_size: usize) -> Self {
        Self {
            board_size,
            cells: Vec::new(),
        }
    }

    pub fn is_full(&self) -> bool {
        self.cells.len() == self.board_size * self.board_size
    }

    pub fn add_row(&mut self, row: &[usize]) {
        let next_row: usize = (self.cells.len() + self.board_size) / self.board_size;

        for (index, value) in row.iter().enumerate() {
            self.cells.push(Cell::new(next_row, index, *value));
        }
    }

    /// the cell with the given value is marked
    /// returns the board score if bingo, otherwise None
    pub fn mark(&mut self, value: usize) -> Option<usize> {
        let cell = self.cells.iter_mut().find(|cell| cell.get_value() == value);

        if let Some(cell) = cell {
            cell.mark();

            let row = cell.get_row();
            let col = cell.get_col();

            if self
                .cells
                .iter()
                .filter(|c| c.get_row() == row)
                .all(|c| c.get_marked())
                || self
                    .cells
                    .iter()
                    .filter(|c| c.get_col() == col)
                    .all(|c| c.get_marked())
            {
                return Some(self.score());
            }
        }

        None
    }

    fn score(&self) -> usize {
        self.cells
            .iter()
            .filter_map(|c| {
                if !c.get_marked() {
                    Some(c.get_value())
                } else {
                    None
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mark() {
        let mut board = Board::new(5);

        board.add_row((1..=5).collect::<Vec<usize>>().as_slice());
        board.add_row((6..=10).collect::<Vec<usize>>().as_slice());
        board.add_row((11..=15).collect::<Vec<usize>>().as_slice());
        board.add_row((16..=20).collect::<Vec<usize>>().as_slice());
        board.add_row((21..=25).collect::<Vec<usize>>().as_slice());

        assert!(board.is_full());

        assert_eq!(board.mark(1), None);
        assert_eq!(board.mark(2), None);
        assert_eq!(board.mark(3), None);
        assert_eq!(board.mark(4), None);
        assert_eq!(board.mark(9), None);
        assert_eq!(board.mark(14), None);
        assert_eq!(board.mark(19), None);
        assert_eq!(board.mark(24), Some(249));
    }
}
