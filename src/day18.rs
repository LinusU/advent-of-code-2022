use std::{collections::BTreeSet, num::ParseIntError, str::FromStr};

use aoc_runner_derive::aoc;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pos(u16);

impl Pos {
    fn neighbours(&self) -> Vec<Pos> {
        vec![
            Pos(self.0 - 1),
            Pos(self.0 + 1),
            Pos(self.0 - (1 << 5)),
            Pos(self.0 + (1 << 5)),
            Pos(self.0 - (1 << 10)),
            Pos(self.0 + (1 << 10)),
        ]
    }
}

impl FromStr for Pos {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',').map(|s| s.parse::<u16>());

        // Add one to avoid having to deal with subtracting 1 from 0 in the neighbours function
        Ok(Pos((parts.next().unwrap()? + 1)
            | (parts.next().unwrap()? + 1) << 5
            | (parts.next().unwrap()? + 1) << 10))
    }
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> Result<usize, ParseIntError> {
    let cubes = input
        .lines()
        .map(|s| s.parse::<Pos>())
        .collect::<Result<BTreeSet<_>, _>>()?;

    let mut result = 0;

    for cube in cubes.iter() {
        result += 6;

        for neighbour in cube.neighbours() {
            if cubes.contains(&neighbour) {
                result -= 1;
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        let result = super::part1("1,1,1");
        assert_eq!(result, Ok(6));
    }

    #[test]
    fn test_case_2() {
        let result = super::part1("1,1,1\n2,1,1");
        assert_eq!(result, Ok(10));
    }

    #[test]
    fn test_case_3() {
        let result = super::part1("2,2,2\n1,2,2\n3,2,2\n2,1,2\n2,3,2\n2,2,1\n2,2,3\n2,2,4\n2,2,6\n1,2,5\n3,2,5\n2,1,5\n2,3,5\n");
        assert_eq!(result, Ok(64));
    }
}
