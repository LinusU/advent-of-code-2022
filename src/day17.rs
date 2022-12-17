use std::{fmt::Debug, num::ParseIntError};

use aoc_runner_derive::aoc;

const WIDTH: usize = 7;

#[repr(u8)]
enum Steam {
    Left = b'<',
    Right = b'>',
}

struct Board {
    data: Vec<bool>,
}

impl Board {
    fn new() -> Self {
        Self { data: vec![] }
    }

    fn max_y(&self) -> usize {
        (self.data.len() + WIDTH - 1) / WIDTH
    }

    fn is_free(&self, x: usize, y: usize) -> bool {
        !self.is_occupied(x, y)
    }

    fn is_occupied(&self, x: usize, y: usize) -> bool {
        *self.data.get(y * WIDTH + x).unwrap_or(&false)
    }

    fn occupy(&mut self, x: usize, y: usize) {
        let pos = y * WIDTH + x;

        if self.data.len() <= pos {
            self.data.resize(pos, false);
            self.data.push(true);
        } else {
            self.data[pos] = true;
        }
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "+-------+")?;

        for y in (0..=self.max_y()).rev() {
            writeln!(
                f,
                "|{}{}{}{}{}{}{}|",
                if self.is_free(0, y) { '.' } else { '#' },
                if self.is_free(1, y) { '.' } else { '#' },
                if self.is_free(2, y) { '.' } else { '#' },
                if self.is_free(3, y) { '.' } else { '#' },
                if self.is_free(4, y) { '.' } else { '#' },
                if self.is_free(5, y) { '.' } else { '#' },
                if self.is_free(6, y) { '.' } else { '#' },
            )?;
        }

        writeln!(f, "+-------+")
    }
}

#[derive(Debug, Clone, Copy)]
enum Shape {
    Horizont,
    Plus,
    Corner,
    Line,
    Cube,
}

impl Shape {
    fn all() -> Vec<Shape> {
        vec![
            Shape::Horizont,
            Shape::Plus,
            Shape::Corner,
            Shape::Line,
            Shape::Cube,
        ]
    }

    fn can_be_placed(&self, board: &Board, x: usize, y: usize) -> bool {
        match self {
            Shape::Horizont => {
                if x + 4 > WIDTH {
                    false
                } else {
                    board.is_free(x, y)
                        && board.is_free(x + 1, y)
                        && board.is_free(x + 2, y)
                        && board.is_free(x + 3, y)
                }
            }
            Shape::Plus => {
                if x + 3 > WIDTH {
                    false
                } else {
                    board.is_free(x + 1, y)
                        && board.is_free(x, y + 1)
                        && board.is_free(x + 1, y + 1)
                        && board.is_free(x + 2, y + 1)
                        && board.is_free(x + 1, y + 2)
                }
            }
            Shape::Corner => {
                if x + 3 > WIDTH {
                    false
                } else {
                    board.is_free(x, y)
                        && board.is_free(x + 1, y)
                        && board.is_free(x + 2, y)
                        && board.is_free(x + 2, y + 1)
                        && board.is_free(x + 2, y + 2)
                }
            }
            Shape::Line => {
                if x + 1 > WIDTH {
                    false
                } else {
                    board.is_free(x, y)
                        && board.is_free(x, y + 1)
                        && board.is_free(x, y + 2)
                        && board.is_free(x, y + 3)
                }
            }
            Shape::Cube => {
                if x + 2 > WIDTH {
                    false
                } else {
                    board.is_free(x, y)
                        && board.is_free(x + 1, y)
                        && board.is_free(x, y + 1)
                        && board.is_free(x + 1, y + 1)
                }
            }
        }
    }

    fn place(&self, board: &mut Board, x: usize, y: usize) {
        match self {
            Shape::Horizont => {
                board.occupy(x, y);
                board.occupy(x + 1, y);
                board.occupy(x + 2, y);
                board.occupy(x + 3, y);
            }
            Shape::Plus => {
                board.occupy(x + 1, y);
                board.occupy(x, y + 1);
                board.occupy(x + 1, y + 1);
                board.occupy(x + 2, y + 1);
                board.occupy(x + 1, y + 2);
            }
            Shape::Corner => {
                board.occupy(x, y);
                board.occupy(x + 1, y);
                board.occupy(x + 2, y);
                board.occupy(x + 2, y + 1);
                board.occupy(x + 2, y + 2);
            }
            Shape::Line => {
                board.occupy(x, y);
                board.occupy(x, y + 1);
                board.occupy(x, y + 2);
                board.occupy(x, y + 3);
            }
            Shape::Cube => {
                board.occupy(x, y);
                board.occupy(x + 1, y);
                board.occupy(x, y + 1);
                board.occupy(x + 1, y + 1);
            }
        }
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> Result<usize, ParseIntError> {
    let shapes = Shape::all().into_iter().cycle();
    let mut steam = input
        .trim()
        .as_bytes()
        .iter()
        .map(|b| unsafe { std::mem::transmute::<u8, Steam>(*b) })
        .cycle();

    let mut board = Board::new();

    for shape in shapes.take(2022) {
        let mut x = 2usize;
        let mut y = board.max_y() + 3;

        for steam in steam.by_ref() {
            match steam {
                Steam::Left => {
                    if x > 0 && shape.can_be_placed(&board, x - 1, y) {
                        x -= 1;
                    }
                }
                Steam::Right => {
                    if shape.can_be_placed(&board, x + 1, y) {
                        x += 1;
                    }
                }
            }

            if y > 0 && shape.can_be_placed(&board, x, y - 1) {
                y -= 1;
            } else {
                shape.place(&mut board, x, y);
                break;
            }
        }
    }

    Ok(board.max_y())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        let result = super::part1(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
        assert_eq!(result, Ok(3068));
    }
}
