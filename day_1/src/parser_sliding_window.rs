use std::{cmp::Ordering, collections::VecDeque, num::ParseIntError};

pub struct ParserSlidingWindow {
    previous_sum: Option<u64>,
    increases: u64,
    window: VecDeque<u64>,
}

impl ParserSlidingWindow {
    pub fn new() -> Self {
        Self {
            previous_sum: None,
            increases: 0,
            window: VecDeque::new(),
        }
    }

    pub fn parse(&mut self, line: &str) -> Result<(), ParseIntError> {
        let value = line.parse::<u64>()?;

        self.window.push_back(value);

        match self.window.len().cmp(&3) {
            Ordering::Less => return Ok(()),
            Ordering::Greater => self.window.pop_front(),
            Ordering::Equal => None,
        };

        let sum: u64 = self.window.iter().sum();

        if let Some(previous) = self.previous_sum {
            if sum > previous {
                self.increases += 1;
            }
        }

        self.previous_sum.replace(sum);

        Ok(())
    }

    pub fn get_increases(&self) -> u64 {
        self.increases
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut parser = ParserSlidingWindow::new();

        for line in vec![
            "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
        ] {
            parser.parse(line).unwrap();
        }

        assert_eq!(parser.get_increases(), 5);
    }
}
