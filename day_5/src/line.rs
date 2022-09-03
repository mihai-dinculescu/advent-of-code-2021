use std::str::FromStr;

use anyhow::Context;

use crate::point::Point;

#[derive(Debug, PartialEq, Eq)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(start: Point, end: Point) -> Self {
        Line { start, end }
    }

    pub fn is_diagonal(&self) -> bool {
        self.start.x != self.end.x && self.start.y != self.end.y
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(" -> ");

        let start = parts.next().context("failed to find Line Start")?.parse()?;
        let end = parts.next().context("failed to find Line End")?.parse()?;

        Ok(Line::new(start, end))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_info() {
        let line = Line::from_str("223,805 -> 223,548").unwrap();
        assert_eq!(line, Line::new(Point::new(223, 805), Point::new(223, 548)));
    }

    #[test]
    fn test_is_diagonal() {
        assert_eq!(
            Line::new(Point::new(0, 9), Point::new(5, 9)).is_diagonal(),
            false
        );
        assert_eq!(
            Line::new(Point::new(8, 0), Point::new(0, 8)).is_diagonal(),
            true
        );
    }
}
