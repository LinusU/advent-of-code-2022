use std::fmt::Debug;
use std::num::ParseIntError;

use aoc_runner_derive::aoc;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{map, map_res},
    multi::separated_list0,
    sequence::{delimited, separated_pair},
    IResult,
};

enum Value {
    List(Vec<Value>),
    Number(u64),
}

impl Value {
    fn from_int_str(s: &str) -> Result<Self, ParseIntError> {
        Ok(Value::Number(s.parse::<u64>()?))
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::List(v) => {
                write!(f, "[")?;
                for (i, e) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{:?}", e)?;
                }
                write!(f, "]")
            }
            Value::Number(n) => write!(f, "{}", n),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::List(left), Value::List(right)) => left == right,
            (Value::Number(left), Value::Number(right)) => left == right,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::List(left), Value::List(right)) => left.partial_cmp(right),
            (Value::Number(left), Value::Number(right)) => left.partial_cmp(right),
            (Value::List(left), Value::Number(right)) => {
                left.partial_cmp(&vec![Value::Number(*right)])
            }
            (Value::Number(left), Value::List(right)) => {
                vec![Value::Number(*left)].partial_cmp(right)
            }
        }
    }
}

struct Pair {
    left: Vec<Value>,
    right: Vec<Value>,
}

impl Debug for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, e) in self.left.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "{:?}", e)?;
        }
        write!(f, "]\n[")?;
        for (i, e) in self.right.iter().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "{:?}", e)?;
        }
        write!(f, "]")
    }
}

fn parse_value_list(i: &str) -> IResult<&str, Vec<Value>> {
    delimited(tag("["), separated_list0(tag(","), parse_value), tag("]"))(i)
}

fn parse_value(i: &str) -> IResult<&str, Value> {
    alt((
        map(parse_value_list, Value::List),
        map_res(digit1, Value::from_int_str),
    ))(i)
}

fn parse_pair(i: &str) -> IResult<&str, Pair> {
    map(
        separated_pair(parse_value_list, char('\n'), parse_value_list),
        |(left, right)| Pair { left, right },
    )(i)
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> Result<usize, ParseIntError> {
    let pairs = input.split("\n\n").map(|s| parse_pair(s).unwrap().1);

    let mut result = 0;

    for (pair_idx, pair) in pairs.enumerate() {
        if pair.left < pair.right {
            result += pair_idx + 1;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        let result = super::part1("[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n\n[9]\n[[8,7,6]]\n\n[[4,4],4,4]\n[[4,4],4,4,4]\n\n[7,7,7,7]\n[7,7,7]\n\n[]\n[3]\n\n[[[]]]\n[[]]\n\n[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]");
        assert_eq!(result, Ok(13));
    }
}
