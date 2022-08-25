use std::str::FromStr;

use anyhow::Context;

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Forward(u64),
    Down(u64),
    Up(u64),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let command = split.next().context("command not found")?;
        let value = split.next().context("value not found")?.parse::<u64>()?;

        match command {
            "forward" => Ok(Command::Forward(value)),
            "down" => Ok(Command::Down(value)),
            "up" => Ok(Command::Up(value)),
            _ => Err(anyhow::anyhow!("unrecognized command {command}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forward_try_into() {
        let command = Command::from_str("forward 8").unwrap();
        assert_eq!(command, Command::Forward(8));
    }

    #[test]
    fn test_down_try_into() {
        let command = Command::from_str("down 5").unwrap();
        assert_eq!(command, Command::Down(5));
    }

    #[test]
    fn test_up_try_into() {
        let command = Command::from_str("up 3").unwrap();
        assert_eq!(command, Command::Up(3));
    }
}
