use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

mod parser;

use parser::Parser;

// https://adventofcode.com/2021/day/3
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day_3/input.txt")?;
    let buffer = BufReader::new(file);

    let mut parser = Parser::<12>::new();

    for line in buffer.lines() {
        let line = line?;
        parser.parse_line(&line)?;
    }

    println!("part 1: {}", parser.gamma_rate() * parser.epsilon_rate());
    println!(
        "part 2: {}",
        parser.oxygen_generator_rating()? * parser.co2_scrubber_rating()?
    );

    Ok(())
}
