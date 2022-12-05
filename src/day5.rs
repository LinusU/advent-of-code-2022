use std::num::ParseIntError;

use aoc_runner_derive::aoc;

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
        let mut words = instruction.split_whitespace();

        assert!(words.next().unwrap() == "move");
        let len = words.next().unwrap().parse::<usize>()?;

        assert!(words.next().unwrap() == "from");
        let from = words.next().unwrap().parse::<usize>()?;

        assert!(words.next().unwrap() == "to");
        let to = words.next().unwrap().parse::<usize>()?;

        for _ in 0..len {
            let c = stacks[from - 1].pop().unwrap();

            stacks[to - 1].push(c);
        }
    }

    let mut result = String::new();

    for stack in stacks {
        result.push(*stack.last().unwrap());
    }

    assert_ne!(result, "GGNPJBTTR");

    Ok(result)
}
