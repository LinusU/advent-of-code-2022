use std::{char, collections::HashSet, str::FromStr};

use aoc_runner_derive::aoc;
use itertools::{chain, Itertools};

struct Rucksack {
    compartments: (HashSet<char>, HashSet<char>),
}

impl Rucksack {
    fn priority(&self) -> u64 {
        let Some(common) = self.compartments.0.iter().find(|c| self.compartments.1.contains(*c)) else {
            panic!("No common character found");
        };

        common.priority()
    }

    fn contains(&self, c: &char) -> bool {
        self.compartments.0.contains(c) || self.compartments.1.contains(c)
    }

    fn iter(&self) -> impl Iterator<Item = &char> {
        chain!(self.compartments.0.iter(), self.compartments.1.iter())
    }
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert!(s.len() % 2 == 0);

        let (first, second) = s.split_at(s.len() / 2);

        Ok(Rucksack {
            compartments: (first.chars().collect(), second.chars().collect()),
        })
    }
}

trait Priority {
    fn priority(&self) -> u64;
}

impl Priority for char {
    fn priority(&self) -> u64 {
        match self {
            'a'..='z' => 1 + (*self as u64 - 'a' as u64),
            'A'..='Z' => 27 + (*self as u64 - 'A' as u64),
            _ => panic!("Invalid input"),
        }
    }
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u64 {
    input
        .split_whitespace()
        .map(|line| line.parse::<Rucksack>().unwrap())
        .fold(0, |acc, rucksack| acc + rucksack.priority())
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u64 {
    input
        .split_whitespace()
        .map(|line| line.parse::<Rucksack>().unwrap())
        .tuples::<(_, _, _)>()
        .map(|rucksacks| {
            let Some(common) = rucksacks.0.iter().find(|c| {
                rucksacks.1.contains(c) && rucksacks.2.contains(c)
            }) else {
                panic!("No common character found");
            };

            common.priority()
        })
        .sum()
}
