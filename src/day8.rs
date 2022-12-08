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

    fn iter_pos(&self) -> impl DoubleEndedIterator<Item = (usize, usize)> + '_ {
        (0..self.height()).flat_map(|y| (0..self.width()).map(move |x| (x, y)))
    }

    fn get(&self, x: usize, y: usize) -> &Tree {
        assert!(x < self.width());
        assert!(y < self.height());

        &self.trees[y * self.width + x]
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

#[aoc(day8, part2)]
pub fn part2(input: &str) -> Result<u64, ParseIntError> {
    let forest = input.parse::<Forest>()?;

    let width = forest.width();
    let height = forest.height();

    Ok(forest
        .iter_pos()
        .map(|(x, y)| {
            let house_height = forest.get(x, y).height;
            let mut score = (0u64, 0u64, 0u64, 0u64);

            for delta in 1..=x {
                score.0 += 1;

                if forest.get(x - delta, y).height >= house_height {
                    break;
                }
            }

            for delta in 1..(width - x) {
                score.1 += 1;

                if forest.get(x + delta, y).height >= house_height {
                    break;
                }
            }

            for delta in 1..=y {
                score.2 += 1;

                if forest.get(x, y - delta).height >= house_height {
                    break;
                }
            }

            for delta in 1..(height - y) {
                score.3 += 1;

                if forest.get(x, y + delta).height >= house_height {
                    break;
                }
            }

            score.0 * score.1 * score.2 * score.3
        })
        .max()
        .unwrap_or(0))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        let result = super::part1("30373\n25512\n65332\n33549\n35390");
        assert_eq!(result, Ok(21));
    }

    #[test]
    fn test_case_2() {
        let result = super::part2("30373\n25512\n65332\n33549\n35390");
        assert_eq!(result, Ok(8));
    }
}
