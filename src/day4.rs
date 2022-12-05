use std::{num::ParseIntError, str::FromStr};

use aoc_runner_derive::aoc;

struct Range {
    start: u64,
    end: u64,
}

impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_pos = s.chars().position(|c| c == '-').unwrap();
        let (start, end) = s.split_at(split_pos);

        Ok(Range {
            start: start.parse()?,
            end: end[1..].parse()?,
        })
    }
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> u64 {
    input
        .split_whitespace()
        .map(|line| {
            let split_pos = line.chars().position(|c| c == ',').unwrap();
            let (first, second) = line.split_at(split_pos);

            let first = first.parse::<Range>().unwrap();
            let second = second[1..].parse::<Range>().unwrap();

            if first.start <= second.start && first.end >= second.end {
                return 1;
            }

            if second.start <= first.start && second.end >= first.end {
                return 1;
            }

            0
        })
        .sum()
}
