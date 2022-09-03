use std::str::FromStr;

use anyhow::Context;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

impl FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().context("failed to find Point X")?.parse()?;
        let y = parts.next().context("failed to find Point Y")?.parse()?;

        Ok(Point::new(x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_info() {
        let point = Point::from_str("0,9").unwrap();
        assert_eq!(point, Point::new(0, 9));
    }
}
