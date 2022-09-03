use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

mod line;
mod parser;
mod point;

use line::Line;
use parser::Parser;

// https://adventofcode.com/2021/day/5
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day_5/input.txt")?;
    let buffer = BufReader::new(file);

    let mut parser = Parser::new();
    let mut diagonal_lines = Vec::new();

    for line in buffer.lines() {
        if let Ok(line) = line.unwrap().parse::<Line>() {
            if !line.is_diagonal() {
                parser.parse(&line);
            } else {
                diagonal_lines.push(line);
            }
        }
    }

    println!("part 1: {}", parser.dangerous_areas());

    for line in diagonal_lines {
        parser.parse(&line);
    }

    println!("part 2: {}", parser.dangerous_areas());

    Ok(())
}
