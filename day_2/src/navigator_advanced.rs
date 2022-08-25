use crate::command::Command;

pub struct NavigatorAdvanced {
    horizontal: u64,
    depth: u64,
    aim: u64,
}

impl NavigatorAdvanced {
    pub fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    pub fn navigate(&mut self, command: &Command) {
        match command {
            Command::Forward(value) => {
                self.horizontal += value;
                self.depth += self.aim * value;
            }
            Command::Down(value) => self.aim += value,
            Command::Up(value) => self.aim -= value,
        }
    }

    pub fn get_horizontal(&self) -> u64 {
        self.horizontal
    }

    pub fn get_depth(&self) -> u64 {
        self.depth
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    const INPUT: [&str; 6] = [
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    #[test]
    fn test_horizontal() {
        let mut navigator = NavigatorAdvanced::new();

        for input in INPUT {
            let command = Command::from_str(input).unwrap();
            navigator.navigate(&command);
        }

        assert_eq!(navigator.get_horizontal(), 15);
    }

    #[test]
    fn test_depth() {
        let mut navigator = NavigatorAdvanced::new();

        for input in INPUT {
            let command = Command::from_str(input).unwrap();
            navigator.navigate(&command);
        }

        assert_eq!(navigator.get_depth(), 60);
    }
}
