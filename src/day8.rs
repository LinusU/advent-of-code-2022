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

struct Forest {
    trees: Vec<Tree>,
    width: usize,
}

impl Forest {
    fn height(&self) -> usize {
        self.trees.len() / self.width
    }

    fn width(&self) -> usize {
        self.width
    }

    fn iter(&self) -> impl DoubleEndedIterator<Item = &Tree> {
        self.trees.iter()
    }

    fn iter_col_mut(&mut self, col: usize) -> impl DoubleEndedIterator<Item = &mut Tree> {
        assert!(col < self.width());

        self.trees.iter_mut().skip(col).step_by(self.width)
    }

    fn iter_row_mut(&mut self, row: usize) -> impl DoubleEndedIterator<Item = &mut Tree> {
        assert!(row < self.height());

        self.trees
            .iter_mut()
            .skip(row * self.width)
            .take(self.width)
    }
}

impl FromStr for Forest {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.find('\n').unwrap_or(s.len());

        let trees = s
            .lines()
            .flat_map(|line| line.chars().map(|c| String::from(c).parse::<Tree>()))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Forest { trees, width })
    }
}

#[aoc(day8, part1)]
pub fn part1(input: &str) -> Result<u64, ParseIntError> {
    let mut forest = input.parse::<Forest>()?;

    let width = forest.width();
    let height = forest.height();

    for x in 0..width {
        let mut h = -1;

        for tree in forest.iter_col_mut(x) {
            if tree.height > h {
                tree.visible = true;
                h = tree.height;
            }
        }

        let mut h = -1;

        for tree in forest.iter_col_mut(x).rev() {
            if tree.height > h {
                tree.visible = true;
                h = tree.height;
            }
        }
    }

    for y in 0..height {
        let mut h = -1;

        for tree in forest.iter_row_mut(y) {
            if tree.height > h {
                tree.visible = true;
                h = tree.height;
            }
        }

        let mut h = -1;

        for tree in forest.iter_row_mut(y).rev() {
            if tree.height > h {
                tree.visible = true;
                h = tree.height;
            }
        }
    }

    Ok(forest.iter().map(|t| t.visible as u64).sum())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        let result = super::part1("30373\n25512\n65332\n33549\n35390");
        assert_eq!(result, Ok(21));
    }
}
