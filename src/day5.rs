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

#[aoc(day5, part1)]
pub fn part1(input: &str) -> Result<String, ParseIntError> {
    let mut lines = input.lines();

    let mut boxes = Vec::<&str>::new();
    let mut width: usize = 0;

    loop {
        let line = lines.next().unwrap();

        if line.starts_with(" 1") {
            assert!((line.len() + 1) % 4 == 0);
            width = (line.len() + 1) / 4;
            continue;
        }

        if line.is_empty() {
            break;
        }

        boxes.push(line);
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

    for instruction in lines {
        let instruction = instruction.parse::<Instruction>()?;

        for _ in 0..instruction.n {
            let c = stacks[instruction.from].pop().unwrap();

            stacks[instruction.to].push(c);
        }
    }

    let mut result = String::new();

    for stack in stacks {
        result.push(*stack.last().unwrap());
    }

    assert_ne!(result, "GGNPJBTTR");

    Ok(result)
}
