use std::cmp::Ordering;

use anyhow::Context;

/// Using const generics and arrays mostly for the learning opportunity.
pub struct Parser<const N: usize> {
    bit_counts: [u32; N],
    count: u32,
    records: Vec<[u32; N]>,
}

impl<const N: usize> Parser<N> {
    pub fn new() -> Self {
        Self {
            bit_counts: [0; N],
            count: 0,
            records: Vec::new(),
        }
    }

    pub fn parse_line(&mut self, input: &str) -> Result<(), anyhow::Error> {
        let allowed = [0, 1];

        let digits = input
            .chars()
            // parse from char to number
            .filter_map(|ch| ch.to_digit(10))
            // validate that the digit is in the allowed range
            .filter(|digit| allowed.contains(digit))
            .collect::<Vec<u32>>();

        // validate the length of the input
        if digits.len() != N {
            return Err(anyhow::anyhow!("invalid input length or content"));
        }

        let digits: [u32; N] = digits
            .try_into()
            .ok()
            .context("failed to convert to an array")?;

        // add to bit_counts
        for (i, digit) in digits.into_iter().enumerate() {
            self.bit_counts[i] += digit;
        }

        // push to records for oxygen and CO2 scrubber ratings
        self.records.push(digits);

        self.count += 1;

        Ok(())
    }

    pub fn gamma_rate(&self) -> u32 {
        let mid = self.count / 2;
        let bits = self.bit_counts.map(|count| if count > mid { 1 } else { 0 });

        Self::bits_to_number(&bits)
    }

    pub fn epsilon_rate(&self) -> u32 {
        let mid = self.count / 2;
        let bits = self.bit_counts.map(|count| if count < mid { 1 } else { 0 });

        Self::bits_to_number(&bits)
    }

    pub fn oxygen_generator_rating(&self) -> Result<u32, anyhow::Error> {
        let records: Vec<&[u32; N]> = self.records.iter().collect();
        let record = Self::filter_list(records, 0, Ordering::Greater)?;

        Ok(Self::bits_to_number(record))
    }

    pub fn co2_scrubber_rating(&self) -> Result<u32, anyhow::Error> {
        let records: Vec<&[u32; N]> = self.records.iter().collect();
        let record = Self::filter_list(records, 0, Ordering::Less)?;

        Ok(Self::bits_to_number(record))
    }

    fn filter_list(
        list: Vec<&[u32; N]>,
        index: usize,
        ordering: Ordering,
    ) -> Result<&[u32; N], anyhow::Error> {
        let bit_1_count = list.iter().filter(|item| item[index] == 1).count();
        let bit_0_count = list.iter().filter(|item| item[index] == 0).count();

        let bit = match ordering {
            Ordering::Greater => match bit_1_count.cmp(&bit_0_count) {
                Ordering::Greater => 1,
                Ordering::Less => 0,
                Ordering::Equal => 1,
            },
            Ordering::Less => match bit_1_count.cmp(&bit_0_count) {
                Ordering::Greater => 0,
                Ordering::Less => 1,
                Ordering::Equal => 0,
            },
            Ordering::Equal => return Err(anyhow::anyhow!("invalid ordering parameter")),
        };

        let new_list: Vec<&[u32; N]> = list.into_iter().filter(|item| item[index] == bit).collect();

        match new_list.len().cmp(&1) {
            Ordering::Equal => Ok(new_list[0]),
            Ordering::Greater => Self::filter_list(new_list, index + 1, ordering),
            Ordering::Less => Err(anyhow::anyhow!("failed to filter down to a single record")),
        }
    }

    fn bits_to_number(bits: &[u32]) -> u32 {
        let mut output = 0;

        for (i, bit) in bits.iter().rev().enumerate() {
            output += bit * 2u32.pow(i as u32);
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: [&str; 12] = [
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    #[test]
    fn test_gamma_rate() {
        let mut parser = Parser::<5>::new();

        for line in INPUT {
            parser.parse_line(line).unwrap();
        }

        assert_eq!(parser.gamma_rate(), 22);
    }

    #[test]
    fn test_epsilon_rate() {
        let mut parser = Parser::<5>::new();

        for line in INPUT {
            parser.parse_line(line).unwrap();
        }

        assert_eq!(parser.epsilon_rate(), 9);
    }

    #[test]
    fn test_oxygen_generator_rating() {
        let mut parser = Parser::<5>::new();

        for line in INPUT {
            parser.parse_line(line).unwrap();
        }

        assert_eq!(parser.oxygen_generator_rating().unwrap(), 23);
    }

    #[test]
    fn test_co2_scrubber_rating() {
        let mut parser = Parser::<5>::new();

        for line in INPUT {
            parser.parse_line(line).unwrap();
        }

        assert_eq!(parser.co2_scrubber_rating().unwrap(), 10);
    }
}
