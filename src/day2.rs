use std::str::FromStr;

use aoc_runner_derive::aoc;

enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            "X" => Ok(Shape::Rock),
            "Y" => Ok(Shape::Paper),
            "Z" => Ok(Shape::Scissors),
            _ => panic!("Invalid shape"),
        }
    }
}

impl Shape {
    fn outcome(&self, other: &Shape) -> Outcome {
        match (self, other) {
            (Shape::Rock, Shape::Rock) => Outcome::Draw,
            (Shape::Rock, Shape::Paper) => Outcome::Lose,
            (Shape::Rock, Shape::Scissors) => Outcome::Win,
            (Shape::Paper, Shape::Rock) => Outcome::Win,
            (Shape::Paper, Shape::Paper) => Outcome::Draw,
            (Shape::Paper, Shape::Scissors) => Outcome::Lose,
            (Shape::Scissors, Shape::Rock) => Outcome::Lose,
            (Shape::Scissors, Shape::Paper) => Outcome::Win,
            (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
        }
    }

    fn score(&self) -> u64 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl Outcome {
    fn score(&self) -> u64 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u64 {
    input
        .split_whitespace()
        .map(|s| s.parse::<Shape>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .chunks_exact(2)
        .fold(0, |acc, shapes| {
            let [opponent, player] = shapes else {
                panic!("Invalid input")
            };

            acc + player.score() + player.outcome(opponent).score()
        })
}
