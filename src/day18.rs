use std::{collections::BTreeSet, num::ParseIntError, str::FromStr};

use aoc_runner_derive::aoc;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pos(u16);

impl Pos {
    fn new(x: u16, y: u16, z: u16) -> Pos {
        Pos(x | y << 5 | z << 10)
    }

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

    fn is_edge(&self) -> bool {
        (self.0 & 0b000000000011111) == 0
            || (self.0 & 0b000000000011111) == 31
            || (self.0 & 0b000001111100000) >> 5 == 0
            || (self.0 & 0b000001111100000) >> 5 == 31
            || (self.0 & 0b111110000000000) >> 10 == 0
            || (self.0 & 0b111110000000000) >> 10 == 31
    }
}

impl FromStr for Pos {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',').map(|s| s.parse::<u16>());

        // Offset a bit to avoid dealing with 0/1 values in rest of code
        Ok(Pos::new(
            parts.next().unwrap()? + 3,
            parts.next().unwrap()? + 3,
            parts.next().unwrap()? + 3,
        ))
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

#[aoc(day18, part2)]
pub fn part2(input: &str) -> Result<usize, ParseIntError> {
    let cubes = input
        .lines()
        .map(|s| s.parse::<Pos>())
        .collect::<Result<BTreeSet<_>, _>>()?;

    let mut outside = BTreeSet::<Pos>::new();
    let mut queue = Vec::<Pos>::new();

    outside.insert(Pos::new(1, 1, 1));
    queue.push(Pos::new(1, 1, 1));

    while let Some(pos) = queue.pop() {
        for neighbour in pos.neighbours() {
            if outside.contains(&neighbour) {
                continue;
            }

            if cubes.contains(&neighbour) {
                continue;
            }

            outside.insert(neighbour);

            if !neighbour.is_edge() {
                queue.push(neighbour);
            }
        }
    }

    let mut result = 0;

    for cube in cubes.iter() {
        for neighbour in cube.neighbours() {
            if outside.contains(&neighbour) {
                result += 1;
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

    #[test]
    fn test_case_4() {
        let result = super::part2("1,1,1");
        assert_eq!(result, Ok(6));
    }

    #[test]
    fn test_case_5() {
        let result = super::part2("1,1,1\n2,1,1");
        assert_eq!(result, Ok(10));
    }

    #[test]
    fn test_case_6() {
        let result = super::part2("2,2,2\n1,2,2\n3,2,2\n2,1,2\n2,3,2\n2,2,1\n2,2,3\n2,2,4\n2,2,6\n1,2,5\n3,2,5\n2,1,5\n2,3,5\n");
        assert_eq!(result, Ok(58));
    }
}
