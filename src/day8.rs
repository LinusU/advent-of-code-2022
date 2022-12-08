use std::{num::ParseIntError, str::FromStr};

use aoc_runner_derive::aoc;

struct Tree {
    height: i8,
    visible: bool,
}

impl FromStr for Tree {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Tree {
            height: s.parse()?,
            visible: false,
        })
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> Result<u64, ParseIntError> {
    let mut trees = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| String::from(c).parse::<Tree>())
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    for line in trees.iter_mut() {
        let mut h = -1;

        for tree in line.iter_mut() {
            if tree.height > h {
                tree.visible = true;
                h = tree.height;
            }
        }

        let mut h = -1;

        for tree in line.iter_mut().rev() {
            if tree.height > h {
                tree.visible = true;
                h = tree.height;
            }
        }
    }

    let width = trees[0].len();

    for x in 0..width {
        let mut h = -1;

        for tree in trees.iter_mut() {
            if tree[x].height > h {
                tree[x].visible = true;
                h = tree[x].height;
            }
        }

        let mut h = -1;

        for tree in trees.iter_mut().rev() {
            if tree[x].height > h {
                tree[x].visible = true;
                h = tree[x].height;
            }
        }
    }

    Ok(trees.iter().flatten().map(|t| t.visible as u64).sum())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        let result = super::part1("30373\n25512\n65332\n33549\n35390");
        assert_eq!(result, Ok(21));
    }
}
