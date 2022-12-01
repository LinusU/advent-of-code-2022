use std::num::ParseIntError;

use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> Result<u64, ParseIntError> {
    let elves = input
        .split("\n\n")
        .map(|elf| elf.split_whitespace().map(|s| s.parse::<u64>()).sum())
        .collect::<Result<Vec<u64>, _>>()?;

    Ok(*elves.iter().max().expect("No elves"))
}
