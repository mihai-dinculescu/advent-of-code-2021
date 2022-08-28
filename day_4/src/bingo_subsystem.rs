use std::collections::VecDeque;

use anyhow::Context;

use crate::board::Board;

#[derive(Debug, Clone)]
pub struct BingoSubsystem {
    board_size: usize,
    future_draws: VecDeque<usize>,
    boards: Vec<Board>,
}

impl BingoSubsystem {
    pub fn new(board_size: usize) -> Self {
        Self {
            board_size,
            future_draws: VecDeque::new(),
            boards: Vec::new(),
        }
    }

    pub fn parse_line(&mut self, input: &str) -> Result<(), anyhow::Error> {
        if self.future_draws.is_empty() {
            let values = input
                .split(',')
                .map(|value| value.parse::<usize>())
                .collect::<Result<VecDeque<usize>, _>>()?;

            self.future_draws.extend(values);
        } else {
            let input = input.trim();

            if !input.is_empty() {
                if self.boards.is_empty() || self.boards[self.boards.len() - 1].is_full() {
                    self.boards.push(Board::new(self.board_size));
                }

                let board_index = self.boards.len() - 1;
                let board = &mut self.boards[board_index];

                let values = input
                    .split_whitespace()
                    .map(|value| value.trim().parse::<usize>())
                    .collect::<Result<Vec<usize>, _>>()?;

                board.add_row(&values);
            }
        }

        Ok(())
    }

    pub fn all_boards_are_full(&self) -> bool {
        for board in &self.boards {
            if !board.is_full() {
                return false;
            }
        }

        true
    }

    pub fn draw_to_win(mut self) -> Result<usize, anyhow::Error> {
        let value = self
            .future_draws
            .pop_front()
            .context("future draws is empty")?;

        for board in &mut self.boards {
            let score = board.mark(value);

            if let Some(score) = score {
                return Ok(score * value);
            }
        }

        self.draw_to_win()
    }

    pub fn draw_to_lose(mut self) -> Result<usize, anyhow::Error> {
        loop {
            let value = self
                .future_draws
                .pop_front()
                .context("future draws is empty")?;

            match self.boards.len() {
                0 => return Err(anyhow::anyhow!("no boards")),
                1 => {
                    let board = &mut self.boards[0];

                    let score = board.mark(value);

                    if let Some(score) = score {
                        return Ok(score * value);
                    }
                }
                _ => {
                    // this makes the big assumption that the last board
                    // will always finish on a subsequent draw from the others
                    self.boards.retain_mut(|b| b.mark(value).is_none());
                    continue;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufRead, BufReader};

    use super::*;

    const INPUT: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
8  2 23  4 24
21  9 14 16  7
6 10  3 18  5
1 12 20 15 19

3 15  0  2 22
9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7
"#;

    fn build_subsystem() -> BingoSubsystem {
        let reader = BufReader::new(INPUT.as_bytes());
        let mut subsystem = BingoSubsystem::new(5);

        for line in reader.lines() {
            let line = line.unwrap();
            subsystem.parse_line(&line).unwrap();
        }

        subsystem
    }

    #[test]
    fn test_parse() {
        let subsystem = build_subsystem();

        assert_eq!(subsystem.future_draws.len(), 27);
        assert_eq!(subsystem.future_draws[0], 7);
        assert_eq!(subsystem.future_draws[subsystem.future_draws.len() - 1], 1);

        assert_eq!(subsystem.boards.len(), 3);
        assert!(subsystem.all_boards_are_full());
    }

    #[test]
    fn test_draw_to_win() {
        let subsystem = build_subsystem();

        let score = subsystem.draw_to_win().unwrap();
        assert_eq!(score, 4512);
    }

    #[test]
    fn test_draw_to_lose() {
        let subsystem = build_subsystem();

        let score = subsystem.draw_to_lose().unwrap();
        assert_eq!(score, 1924);
    }
}
