use std::{collections::BTreeMap, num::ParseIntError, str::FromStr};

use aoc_runner_derive::aoc;

const ROOT: Name = Name(u32::from_be_bytes(*b"root"));
const ME: Name = Name(u32::from_be_bytes(*b"humn"));

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Name(u32);

impl FromStr for Name {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.as_bytes();

        Ok(Name(u32::from_be_bytes([s[0], s[1], s[2], s[3]])))
    }
}

enum Job {
    Static(u64),
    Add(Name, Name),
    Sub(Name, Name),
    Mul(Name, Name),
    Div(Name, Name),
}

impl Job {
    fn dependencies(&self) -> Option<(Name, Name)> {
        match self {
            Job::Static(_) => None,
            Job::Add(lhs, rhs) => Some((*lhs, *rhs)),
            Job::Sub(lhs, rhs) => Some((*lhs, *rhs)),
            Job::Mul(lhs, rhs) => Some((*lhs, *rhs)),
            Job::Div(lhs, rhs) => Some((*lhs, *rhs)),
        }
    }

    fn value(&self, known: &BTreeMap<Name, u64>) -> Option<u64> {
        match self {
            Job::Static(value) => Some(*value),
            Job::Add(lhs, rhs) => {
                Option::zip(known.get(lhs), known.get(rhs)).map(|(lhs, rhs)| lhs + rhs)
            }
            Job::Sub(lhs, rhs) => {
                Option::zip(known.get(lhs), known.get(rhs)).map(|(lhs, rhs)| lhs - rhs)
            }
            Job::Mul(lhs, rhs) => {
                Option::zip(known.get(lhs), known.get(rhs)).map(|(lhs, rhs)| lhs * rhs)
            }
            Job::Div(lhs, rhs) => {
                Option::zip(known.get(lhs), known.get(rhs)).map(|(lhs, rhs)| lhs / rhs)
            }
        }
    }
}

impl FromStr for Job {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().next().unwrap().is_numeric() {
            Ok(Job::Static(s.parse()?))
        } else {
            match s.chars().nth(5).unwrap() {
                '+' => Ok(Job::Add(s[0..4].parse()?, s[7..11].parse()?)),
                '-' => Ok(Job::Sub(s[0..4].parse()?, s[7..11].parse()?)),
                '*' => Ok(Job::Mul(s[0..4].parse()?, s[7..11].parse()?)),
                '/' => Ok(Job::Div(s[0..4].parse()?, s[7..11].parse()?)),
                _ => panic!("Invalid input"),
            }
        }
    }
}

struct Monkey {
    name: Name,
    job: Job,
}

impl FromStr for Monkey {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Monkey {
            name: s[0..=4].parse()?,
            job: s[6..].parse()?,
        })
    }
}

#[aoc(day21, part1)]
pub fn part1(input: &str) -> Result<u64, ParseIntError> {
    let mut monkeys = input
        .lines()
        .map(Monkey::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    let mut known = BTreeMap::<Name, u64>::new();

    loop {
        monkeys.retain(|monkey| {
            if let Some(value) = monkey.job.value(&known) {
                known.insert(monkey.name, value);
                false
            } else {
                true
            }
        });

        if let Some(result) = known.get(&ROOT) {
            return Ok(*result);
        }
    }
}

#[aoc(day21, part2)]
pub fn part2(input: &str) -> Result<u64, ParseIntError> {
    let mut monkeys = input
        .lines()
        .map(Monkey::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    let mut known = BTreeMap::<Name, u64>::new();
    let mut start_reverse_from = Option::<Name>::None;

    while !known.contains_key(&ROOT) {
        monkeys.retain(|monkey| {
            if monkey.name == ME {
                return true;
            }

            if monkey.name == ROOT {
                let (lhs, rhs) = monkey.job.dependencies().unwrap();

                if let Some(value) = known.get(&lhs).cloned() {
                    known.insert(ROOT, value);
                    known.insert(rhs, value);
                    start_reverse_from = Some(rhs);
                } else if let Some(value) = known.get(&rhs).cloned() {
                    known.insert(ROOT, value);
                    known.insert(lhs, value);
                    start_reverse_from = Some(lhs);
                }

                return true;
            }

            if let Some(value) = monkey.job.value(&known) {
                known.insert(monkey.name, value);
                false
            } else {
                true
            }
        });
    }

    let monkeys = monkeys
        .into_iter()
        .map(|m| (m.name, m))
        .collect::<BTreeMap<_, _>>();

    let mut current = start_reverse_from.unwrap();

    while current != ME {
        let result = known.get(&current).unwrap();
        let monkey = monkeys.get(&current).unwrap();
        let (lhs, rhs) = monkey.job.dependencies().unwrap();
        let (lhs_value, rhs_value) = (known.get(&lhs).cloned(), known.get(&rhs).cloned());

        let value = match (lhs_value, rhs_value, &monkey.job) {
            (Some(x), None, Job::Add(_, _)) => result - x,
            (None, Some(x), Job::Add(_, _)) => result - x,
            (Some(x), None, Job::Sub(_, _)) => x - result,
            (None, Some(x), Job::Sub(_, _)) => result + x,
            (Some(x), None, Job::Mul(_, _)) => result / x,
            (None, Some(x), Job::Mul(_, _)) => result / x,
            (Some(x), None, Job::Div(_, _)) => x / result,
            (None, Some(x), Job::Div(_, _)) => x * result,
            _ => panic!("My algorithm is broken"),
        };

        if lhs_value.is_none() {
            known.insert(lhs, value);
            current = lhs;
        } else {
            known.insert(rhs, value);
            current = rhs;
        }
    }

    Ok(*known.get(&ME).unwrap())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        let result = super::part1("root: pppw + sjmn\ndbpl: 5\ncczh: sllz + lgvd\nzczc: 2\nptdq: humn - dvpt\ndvpt: 3\nlfqf: 4\nhumn: 5\nljgn: 2\nsjmn: drzm * dbpl\nsllz: 4\npppw: cczh / lfqf\nlgvd: ljgn * ptdq\ndrzm: hmdt - zczc\nhmdt: 32");
        assert_eq!(result, Ok(152));
    }

    #[test]
    fn test_case_2() {
        let result = super::part2("root: pppw + sjmn\ndbpl: 5\ncczh: sllz + lgvd\nzczc: 2\nptdq: humn - dvpt\ndvpt: 3\nlfqf: 4\nhumn: 5\nljgn: 2\nsjmn: drzm * dbpl\nsllz: 4\npppw: cczh / lfqf\nlgvd: ljgn * ptdq\ndrzm: hmdt - zczc\nhmdt: 32");
        assert_eq!(result, Ok(301));
    }
}
