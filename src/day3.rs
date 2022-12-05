use std::{char, collections::HashSet, str::FromStr};

use aoc_runner_derive::aoc;

struct Rucksack {
    compartments: (HashSet<char>, HashSet<char>),
}

impl Rucksack {
    fn priority(&self) -> u64 {
        let Some(common) = self.compartments.0.iter().find(|c| self.compartments.1.contains(*c)) else {
            panic!("No common character found");
        };

        match common {
            'a'..='z' => 1 + (*common as u64 - 'a' as u64),
            'A'..='Z' => 27 + (*common as u64 - 'A' as u64),
            _ => panic!("Invalid input"),
        }
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

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u64 {
    input
        .split_whitespace()
        .map(|line| line.parse::<Rucksack>().unwrap())
        .fold(0, |acc, rucksack| acc + rucksack.priority())
}
