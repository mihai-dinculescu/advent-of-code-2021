use std::{collections::HashMap, error::Error, fs};

const PARENT_TIME: i64 = 7;
const CHILD_TIME: i64 = 9;
const PART_1_DAYS: i64 = 80;
const PART_2_DAYS: i64 = 256;

// https://adventofcode.com/2021/day/6
fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("day_6/input.txt")?;
    let input = input.trim();
    let mut cache = HashMap::new();

    let mut count = 0;
    for value in input.split(',') {
        if let Ok(hatch_time) = value.parse::<i64>() {
            count += tick(&mut cache, PART_1_DAYS - hatch_time);
        }
    }
    println!("Part 1: {}", count);

    let mut count = 0;
    for value in input.split(',') {
        if let Ok(hatch_time) = value.parse::<i64>() {
            count += tick(&mut cache, PART_2_DAYS - hatch_time);
        }
    }
    println!("Part 2: {}", count);

    Ok(())
}

fn tick(cache: &mut HashMap<i64, i64>, days_left: i64) -> i64 {
    if days_left <= 0 {
        1
    } else {
        let parent_next = days_left - PARENT_TIME;
        let parent_count = if let Some(&count) = cache.get(&parent_next) {
            count
        } else {
            let count = tick(cache, parent_next);
            cache.insert(parent_next, count);
            count
        };

        let child_next = days_left - CHILD_TIME;
        let child_count = if let Some(&count) = cache.get(&child_next) {
            count
        } else {
            let count = tick(cache, child_next);
            cache.insert(child_next, count);
            count
        };

        parent_count + child_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_fish() {
        let mut count = 0;
        let mut cache = HashMap::new();

        count += tick(&mut cache, 15);

        assert_eq!(count, 5);
    }

    #[test]
    fn test_18_days() {
        let fish_ages = vec![3, 4, 3, 1, 2];
        let mut count = 0;
        let mut cache = HashMap::new();

        for age in fish_ages {
            count += tick(&mut cache, 18 - age);
        }

        assert_eq!(count, 26);
    }

    #[test]
    fn test_80_days() {
        let fish_ages = vec![3, 4, 3, 1, 2];
        let mut count = 0;
        let mut cache = HashMap::new();

        for age in fish_ages {
            count += tick(&mut cache, 80 - age);
        }

        assert_eq!(count, 5934);
    }

    #[test]
    fn test_256_days() {
        let fish_ages = vec![3, 4, 3, 1, 2];
        let mut count = 0;
        let mut cache = HashMap::new();

        for age in fish_ages {
            count += tick(&mut cache, 256 - age);
        }

        assert_eq!(count, 26984457539);
    }
}
