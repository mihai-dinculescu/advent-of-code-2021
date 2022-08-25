use std::num::ParseIntError;

pub struct ParserSimple {
    previous_value: Option<u64>,
    increases: u64,
}

impl ParserSimple {
    pub fn new() -> Self {
        Self {
            previous_value: None,
            increases: 0,
        }
    }

    pub fn parse(&mut self, line: &str) -> Result<(), ParseIntError> {
        let value = line.parse::<u64>()?;

        if let Some(previous) = self.previous_value {
            if value > previous {
                self.increases += 1;
            }
        }

        self.previous_value.replace(value);

        Ok(())
    }

    pub fn get_increases(&self) -> u64 {
        self.increases
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut parser = ParserSimple::new();

        for line in vec![
            "199", "200", "208", "210", "200", "207", "240", "269", "260", "263",
        ] {
            parser.parse(line).unwrap();
        }

        assert_eq!(parser.get_increases(), 7);
    }
}
