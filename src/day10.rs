use std::{num::ParseIntError, str::FromStr};

use aoc_runner_derive::aoc;

enum Instruction {
    Noop,
    AddX(i32),
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            return Ok(Instruction::Noop);
        }

        if let Some(num) = s.strip_prefix("addx ") {
            return Ok(Instruction::AddX(num.parse::<i32>()?));
        }

        panic!("Unknown instruction: {}", s);
    }
}

struct Cpu {
    cycle: usize,
    reg_x: i32,
    part_1_result: i32,
    part_2_result: String,
}

impl Cpu {
    fn new() -> Self {
        Self {
            cycle: 0,
            reg_x: 1,
            part_1_result: 0,
            part_2_result: String::with_capacity(41 * 6),
        }
    }

    fn tick(&mut self) {
        self.cycle += 1;

        if self.cycle % 40 == 20 {
            self.part_1_result += (self.cycle as i32) * self.reg_x;
        }

        let pos = ((self.cycle as i32) - 1) % 40;

        if pos == 0 {
            self.part_2_result.push('\n');
        }

        if self.reg_x >= pos - 1 && self.reg_x <= pos + 1 {
            self.part_2_result.push('#');
        } else {
            self.part_2_result.push('.');
        }
    }

    fn run(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Noop => self.tick(),
            Instruction::AddX(x) => {
                self.tick();
                self.tick();
                self.reg_x += x;
            }
        }
    }
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> Result<i32, ParseIntError> {
    let instructions = input.lines().map(|line| line.parse::<Instruction>());

    let mut cpu = Cpu::new();

    for instruction in instructions {
        cpu.run(&instruction?);
    }

    Ok(cpu.part_1_result)
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> Result<String, ParseIntError> {
    let instructions = input.lines().map(|line| line.parse::<Instruction>());

    let mut cpu = Cpu::new();

    for instruction in instructions {
        cpu.run(&instruction?);
    }

    Ok(cpu.part_2_result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        let result = super::part1("addx 15\naddx -11\naddx 6\naddx -3\naddx 5\naddx -1\naddx -8\naddx 13\naddx 4\nnoop\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx 5\naddx -1\naddx -35\naddx 1\naddx 24\naddx -19\naddx 1\naddx 16\naddx -11\nnoop\nnoop\naddx 21\naddx -15\nnoop\nnoop\naddx -3\naddx 9\naddx 1\naddx -3\naddx 8\naddx 1\naddx 5\nnoop\nnoop\nnoop\nnoop\nnoop\naddx -36\nnoop\naddx 1\naddx 7\nnoop\nnoop\nnoop\naddx 2\naddx 6\nnoop\nnoop\nnoop\nnoop\nnoop\naddx 1\nnoop\nnoop\naddx 7\naddx 1\nnoop\naddx -13\naddx 13\naddx 7\nnoop\naddx 1\naddx -33\nnoop\nnoop\nnoop\naddx 2\nnoop\nnoop\nnoop\naddx 8\nnoop\naddx -1\naddx 2\naddx 1\nnoop\naddx 17\naddx -9\naddx 1\naddx 1\naddx -3\naddx 11\nnoop\nnoop\naddx 1\nnoop\naddx 1\nnoop\nnoop\naddx -13\naddx -19\naddx 1\naddx 3\naddx 26\naddx -30\naddx 12\naddx -1\naddx 3\naddx 1\nnoop\nnoop\nnoop\naddx -9\naddx 18\naddx 1\naddx 2\nnoop\nnoop\naddx 9\nnoop\nnoop\nnoop\naddx -1\naddx 2\naddx -37\naddx 1\naddx 3\nnoop\naddx 15\naddx -21\naddx 22\naddx -6\naddx 1\nnoop\naddx 2\naddx 1\nnoop\naddx -10\nnoop\nnoop\naddx 20\naddx 1\naddx 2\naddx 2\naddx -6\naddx -11\nnoop\nnoop\nnoop");
        assert_eq!(result, Ok(13140));
    }
}
