use std::cmp::Reverse;
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

#[aoc(day1, part2)]
pub fn part2(input: &str) -> Result<u64, ParseIntError> {
    let mut elves = input
        .split("\n\n")
        .map(|elf| elf.split_whitespace().map(|s| s.parse::<u64>()).sum())
        .collect::<Result<Vec<u64>, _>>()?;

    elves.sort_by_key(|w| Reverse(*w));

    Ok(elves[0] + elves[1] + elves[2])
}
