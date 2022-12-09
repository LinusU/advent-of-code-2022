use std::{cmp, collections::HashSet, num::ParseIntError, str::FromStr};

use aoc_runner_derive::aoc;

enum Move {
    Down(i32),
    Left(i32),
    Right(i32),
    Up(i32),
}

impl Move {
    fn direction(&self) -> (i32, i32) {
        match self {
            Move::Down(_) => (0, -1),
            Move::Left(_) => (-1, 0),
            Move::Right(_) => (1, 0),
            Move::Up(_) => (0, 1),
        }
    }

    fn distance(&self) -> i32 {
        match self {
            Move::Down(dist) => *dist,
            Move::Left(dist) => *dist,
            Move::Right(dist) => *dist,
            Move::Up(dist) => *dist,
        }
    }
}

impl FromStr for Move {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, dist) = s.split_once(' ').unwrap();
        let dist = dist.parse::<i32>()?;

        Ok(match dir {
            "D" => Move::Down(dist),
            "L" => Move::Left(dist),
            "R" => Move::Right(dist),
            "U" => Move::Up(dist),
            _ => panic!("Invalid direction"),
        })
    }
}

#[aoc(day9, part1)]
pub fn part1(input: &str) -> Result<usize, ParseIntError> {
    let moves = input
        .lines()
        .map(|s| s.parse::<Move>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut visited = HashSet::<(i32, i32)>::new();

    let mut head = (0, 0);
    let mut tail = (0, 0);

    visited.insert(tail);

    for m in moves {
        let distance = m.distance();
        let (dx, dy) = m.direction();

        for _ in 0..distance {
            head.0 += dx;
            head.1 += dy;

            let touching = cmp::max((head.0 - tail.0).abs(), (head.1 - tail.1).abs()) <= 1;

            if touching {
                continue;
            }

            tail.0 += (head.0 - tail.0).signum();
            tail.1 += (head.1 - tail.1).signum();

            visited.insert(tail);
        }
    }

    Ok(visited.len())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        let result = super::part1("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2\n");
        assert_eq!(result, Ok(13));
    }
}
