use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

mod command;
mod navigator_advanced;
mod navigator_simple;

use crate::{
    command::Command, navigator_advanced::NavigatorAdvanced, navigator_simple::NavigatorSimple,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut navigator_simple = NavigatorSimple::new();
    let mut navigator_advanced = NavigatorAdvanced::new();

    let file = File::open("day_2/input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let command = Command::from_str(&line)?;
        navigator_simple.navigate(&command);
        navigator_advanced.navigate(&command);
    }

    println!(
        "part 1: {}",
        navigator_simple.get_horizontal() * navigator_simple.get_depth()
    );

    println!(
        "part 2: {}",
        navigator_advanced.get_horizontal() * navigator_advanced.get_depth()
    );

    Ok(())
}
