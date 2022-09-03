use std::{cmp, collections::HashMap};

use crate::{line::Line, point::Point};

pub struct Parser {
    vents: HashMap<Point, usize>,
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            vents: HashMap::new(),
        }
    }

    pub fn parse(&mut self, line: &Line) {
        let mut x = line.start.x;
        let mut y = line.start.y;

        loop {
            let point = Point::new(x, y);
            let count = self.vents.entry(point).or_insert(0);
            *count += 1;

            if x == line.end.x && y == line.end.y {
                break;
            }

            match x.cmp(&line.end.x) {
                cmp::Ordering::Less => x += 1,
                cmp::Ordering::Greater => x -= 1,
                cmp::Ordering::Equal => {}
            }

            match y.cmp(&line.end.y) {
                cmp::Ordering::Less => y += 1,
                cmp::Ordering::Greater => y -= 1,
                cmp::Ordering::Equal => {}
            }
        }
    }

    pub fn dangerous_areas(&self) -> usize {
        self.vents.iter().filter(|(_, count)| *count > &1).count()
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufRead, BufReader};

    use super::*;

    const INPUT: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"#;

    #[test]
    fn test_parse_without_diagonal() {
        let reader = BufReader::new(INPUT.as_bytes());
        let mut parser = Parser::new();

        for line in reader.lines() {
            if let Ok(line) = line.unwrap().parse::<Line>() {
                if !line.is_diagonal() {
                    parser.parse(&line);
                }
            }
        }

        assert_eq!(parser.dangerous_areas(), 5);
    }

    #[test]
    fn test_parse_with_diagonal() {
        let reader = BufReader::new(INPUT.as_bytes());
        let mut parser = Parser::new();

        for line in reader.lines() {
            if let Ok(line) = line.unwrap().parse::<Line>() {
                parser.parse(&line);
            }
        }

        assert_eq!(parser.dangerous_areas(), 12);
    }
}
