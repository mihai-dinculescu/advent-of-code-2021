use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

mod parser_simple;
mod parser_sliding_window;

use parser_simple::ParserSimple;
use parser_sliding_window::ParserSlidingWindow;

// https://adventofcode.com/2021/day/1
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day_1/input.txt")?;
    let reader = BufReader::new(file);

    let mut parser_simple = ParserSimple::new();
    let mut parser_sliding_window = ParserSlidingWindow::new();

    for line in reader.lines() {
        let line = line?;
        parser_simple.parse(&line)?;
        parser_sliding_window.parse(&line)?;
    }

    println!("part 1: {}", parser_simple.get_increases());
    println!("part 2: {}", parser_sliding_window.get_increases());

    Ok(())
}
