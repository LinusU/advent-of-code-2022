use std::{num::ParseIntError, str::FromStr};

use aoc_runner_derive::aoc;

struct Instruction {
    n: usize,
    from: usize,
    to: usize,
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();

        assert!(words.next().unwrap() == "move");
        let n = words.next().unwrap().parse::<usize>()?;

        assert!(words.next().unwrap() == "from");
        let from = words.next().unwrap().parse::<usize>()? - 1;

        assert!(words.next().unwrap() == "to");
        let to = words.next().unwrap().parse::<usize>()? - 1;

        Ok(Instruction { n, from, to })
    }
}

struct Ship {
    stacks: Vec<Vec<char>>,
}

impl Ship {
    fn result(&self) -> String {
        let mut result = String::new();

        for stack in &self.stacks {
            result.push(*stack.last().unwrap());
        }

        result
    }
}

impl FromStr for Ship {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut boxes = Vec::<&str>::new();
        let mut width: usize = 0;

        for line in s.lines() {
            if line.starts_with(" 1") {
                assert!((line.len() + 1) % 4 == 0);
                width = (line.len() + 1) / 4;
            } else {
                boxes.push(line);
            }
        }

        boxes.reverse();

        let mut stacks = vec![Vec::<char>::new(); width];

        for line in boxes {
            for (idx, c) in line.char_indices().filter(|(i, _)| i % 4 == 1) {
                if c == ' ' {
                    continue;
                }

                stacks[idx / 4].push(c)
            }
        }

        Ok(Ship { stacks })
    }
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> Result<String, ParseIntError> {
    let split_pos = input.find("\n\n").unwrap() + 2;
    let (ship, instructions) = input.split_at(split_pos);

    let mut ship = ship.parse::<Ship>()?;

    for instruction in instructions.lines() {
        let instruction = instruction.parse::<Instruction>()?;

        for _ in 0..instruction.n {
            let c = ship.stacks[instruction.from].pop().unwrap();

            ship.stacks[instruction.to].push(c);
        }
    }

    let result = ship.result();

    assert_ne!(result, "GGNPJBTTR");
    assert_eq!(result, "QNNTGTPFN");

    Ok(result)
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> Result<String, ParseIntError> {
    let split_pos = input.find("\n\n").unwrap() + 2;
    let (ship, instructions) = input.split_at(split_pos);

    let mut ship = ship.parse::<Ship>()?;

    for instruction in instructions.lines() {
        let instruction = instruction.parse::<Instruction>()?;

        let from_len = ship.stacks[instruction.from].len();
        let boxes = ship.stacks[instruction.from].split_off(from_len - instruction.n);

        ship.stacks[instruction.to].extend(boxes);
    }

    Ok(ship.result())
}
