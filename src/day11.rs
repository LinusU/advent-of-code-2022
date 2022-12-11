use std::{cmp::Reverse, num::ParseIntError, str::FromStr};

use aoc_runner_derive::aoc;

#[derive(Debug)]
enum Operation {
    Add(i32),
    Multiply(i32),
    Square,
}

impl Operation {
    fn apply(&self, x: i32) -> i32 {
        match self {
            Self::Add(y) => x + y,
            Self::Multiply(y) => x * y,
            Self::Square => x * x,
        }
    }
}

impl FromStr for Operation {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(op) = s.trim().strip_prefix("Operation: new = ") else {
            panic!("Malformed input");
        };

        if op == "old * old" {
            Ok(Operation::Square)
        } else if let Some(op) = op.strip_prefix("old * ") {
            Ok(Operation::Multiply(op.parse()?))
        } else if let Some(op) = op.strip_prefix("old + ") {
            Ok(Operation::Add(op.parse()?))
        } else {
            panic!("Malformed input");
        }
    }
}

#[derive(Debug)]
enum Test {
    DivisibleBy(i32),
}

impl Test {
    fn test(&self, x: i32) -> bool {
        match self {
            Self::DivisibleBy(y) => x % y == 0,
        }
    }
}

impl FromStr for Test {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(x) = s.trim().strip_prefix("Test: divisible by ") else {
            panic!("Malformed input");
        };

        Ok(Test::DivisibleBy(x.parse()?))
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i32>,
    op: Operation,
    test: Test,
    true_target: usize,
    false_target: usize,
    inspected: u64,
}

impl Monkey {
    fn target(&self, item: i32) -> usize {
        if self.test.test(item) {
            self.true_target
        } else {
            self.false_target
        }
    }
}

trait Parsing {
    type Err;

    fn parse_starting_items(&self) -> Result<Vec<i32>, Self::Err>;
    fn parse_target(&self) -> Result<usize, Self::Err>;
}

impl Parsing for str {
    type Err = ParseIntError;

    fn parse_starting_items(&self) -> Result<Vec<i32>, Self::Err> {
        let Some(items) = self.trim().strip_prefix("Starting items: ") else {
            panic!("Malformed input");
        };

        items.split(',').map(|s| s.trim().parse::<i32>()).collect()
    }

    fn parse_target(&self) -> Result<usize, Self::Err> {
        if let Some(target) = self.trim().strip_prefix("If true: throw to monkey ") {
            return target.parse();
        }

        if let Some(target) = self.trim().strip_prefix("If false: throw to monkey ") {
            return target.parse();
        }

        panic!("Malformed input");
    }
}

impl FromStr for Monkey {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().skip(1).collect::<Vec<_>>();

        Ok(Self {
            items: lines[0].parse_starting_items()?,
            op: lines[1].parse()?,
            test: lines[2].parse()?,
            true_target: lines[3].parse_target()?,
            false_target: lines[4].parse_target()?,
            inspected: 0,
        })
    }
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> Result<u64, ParseIntError> {
    let mut monkeys = input
        .split("\n\n")
        .map(|input| input.parse::<Monkey>())
        .collect::<Result<Vec<_>, _>>()?;

    for _ in 0..20 {
        for source in 0..monkeys.len() {
            let queue = std::mem::take(&mut monkeys[source].items);

            for item in queue {
                let item = monkeys[source].op.apply(item) / 3;
                let target = monkeys[source].target(item);

                monkeys[source].inspected += 1;
                monkeys[target].items.push(item);
            }
        }
    }

    let mut inspected = monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>();

    inspected.sort_by_key(|w| Reverse(*w));

    Ok(inspected[0] * inspected[1])
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        let result = super::part1("Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n\nMonkey 1:\n  Starting items: 54, 65, 75, 74\n  Operation: new = old + 6\n  Test: divisible by 19\n    If true: throw to monkey 2\n    If false: throw to monkey 0\n\nMonkey 2:\n  Starting items: 79, 60, 97\n  Operation: new = old * old\n  Test: divisible by 13\n    If true: throw to monkey 1\n    If false: throw to monkey 3\n\nMonkey 3:\n  Starting items: 74\n  Operation: new = old + 3\n  Test: divisible by 17\n    If true: throw to monkey 0\n    If false: throw to monkey 1");
        assert_eq!(result, Ok(10605));
    }
}
