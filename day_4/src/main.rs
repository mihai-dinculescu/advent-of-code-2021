use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

mod bingo_subsystem;
mod board;
mod cell;

use bingo_subsystem::BingoSubsystem;

// https://adventofcode.com/2021/day/4
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("day_4/input.txt")?;
    let buffer = BufReader::new(file);

    let mut subsystem = BingoSubsystem::new(5);

    for line in buffer.lines() {
        let line = line?;
        subsystem.parse_line(&line)?;
    }

    assert!(subsystem.all_boards_are_full());

    let score = subsystem.clone().draw_to_win()?;
    println!("part 1: {score}");

    let score = subsystem.draw_to_lose()?;
    println!("part 2: {score}");

    Ok(())
}
