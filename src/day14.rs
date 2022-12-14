use std::{collections::HashSet, num::ParseIntError, str::FromStr};

use aoc_runner_derive::aoc;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: u32,
    y: u32,
}

impl Coord {
    fn add(&self, x: i32, y: i32) -> Coord {
        Coord {
            x: self.x.checked_add_signed(x).unwrap(),
            y: self.y.checked_add_signed(y).unwrap(),
        }
    }
}

impl From<(u32, u32)> for Coord {
    fn from((x, y): (u32, u32)) -> Self {
        Coord { x, y }
    }
}

impl FromStr for Coord {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();

        Ok(Coord {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> Result<u32, ParseIntError> {
    let mut blocked = HashSet::<Coord>::new();
    let mut max_y = 0;

    for line in input.lines() {
        let parts = line
            .split(" -> ")
            .map(Coord::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        max_y = max_y.max(parts.iter().map(|c| c.y).max().unwrap());

        for (a, b) in parts.iter().tuple_windows() {
            if a.x == b.x {
                for y in a.y.min(b.y)..=a.y.max(b.y) {
                    blocked.insert((a.x, y).into());
                }
            } else {
                for x in a.x.min(b.x)..=a.x.max(b.x) {
                    blocked.insert((x, a.y).into());
                }
            }
        }
    }

    let mut result = 0;

    loop {
        let mut sand = Coord { x: 500, y: 0 };

        loop {
            if sand.y > max_y {
                return Ok(result);
            }

            if !blocked.contains(&sand.add(0, 1)) {
                sand = sand.add(0, 1);
                continue;
            }

            if !blocked.contains(&sand.add(-1, 1)) {
                sand = sand.add(-1, 1);
                continue;
            }

            if !blocked.contains(&sand.add(1, 1)) {
                sand = sand.add(1, 1);
                continue;
            }

            blocked.insert(sand);
            result += 1;
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        let result = super::part1("498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9");
        assert_eq!(result, Ok(24));
    }
}
