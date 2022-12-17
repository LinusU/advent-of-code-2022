use std::{fmt::Debug, num::ParseIntError};

use aoc_runner_derive::aoc;

const WIDTH: usize = 7;

const fn mask(value: usize) -> u8 {
    match value {
        0 => 0b01000000,
        1 => 0b00100000,
        2 => 0b00010000,
        3 => 0b00001000,
        4 => 0b00000100,
        5 => 0b00000010,
        6 => 0b00000001,
        _ => panic!("Out of bounds"),
    }
}

#[repr(u8)]
enum Steam {
    Left = b'<',
    Right = b'>',
}

impl From<&u8> for Steam {
    fn from(value: &u8) -> Self {
        match value {
            b'<' => Steam::Left,
            b'>' => Steam::Right,
            _ => panic!("Invalid input {value}"),
        }
    }
}

struct Board {
    data: Vec<u8>,
}

impl Board {
    fn new() -> Self {
        Self { data: vec![] }
    }

    fn max_y(&self) -> usize {
        self.data.len()
    }

    fn is_free(&self, y: usize, mask: u8) -> bool {
        let Some(row) = self.data.get(y) else {
            return true;
        };

        (row & mask) == 0
    }

    fn occupy(&mut self, y: usize, mask: u8) {
        if self.data.len() <= y {
            self.data.resize(y, 0);
            self.data.push(mask);
        } else {
            self.data[y] |= mask;
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
                if self.is_free(y, mask(0)) { '.' } else { '#' },
                if self.is_free(y, mask(1)) { '.' } else { '#' },
                if self.is_free(y, mask(2)) { '.' } else { '#' },
                if self.is_free(y, mask(3)) { '.' } else { '#' },
                if self.is_free(y, mask(4)) { '.' } else { '#' },
                if self.is_free(y, mask(5)) { '.' } else { '#' },
                if self.is_free(y, mask(6)) { '.' } else { '#' },
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
                    board.is_free(y, 0b01111000 >> x)
                }
            }
            Shape::Plus => {
                if x + 3 > WIDTH {
                    false
                } else {
                    board.is_free(y, 0b00100000 >> x)
                        && board.is_free(y + 1, 0b01110000 >> x)
                        && board.is_free(y + 2, 0b00100000 >> x)
                }
            }
            Shape::Corner => {
                if x + 3 > WIDTH {
                    false
                } else {
                    board.is_free(y, 0b01110000 >> x)
                        && board.is_free(y + 1, 0b00010000 >> x)
                        && board.is_free(y + 2, 0b00010000 >> x)
                }
            }
            Shape::Line => {
                if x + 1 > WIDTH {
                    false
                } else {
                    board.is_free(y, 0b01000000 >> x)
                        && board.is_free(y + 1, 0b01000000 >> x)
                        && board.is_free(y + 2, 0b01000000 >> x)
                        && board.is_free(y + 3, 0b01000000 >> x)
                }
            }
            Shape::Cube => {
                if x + 2 > WIDTH {
                    false
                } else {
                    board.is_free(y, 0b01100000 >> x) && board.is_free(y + 1, 0b01100000 >> x)
                }
            }
        }
    }

    fn place(&self, board: &mut Board, x: usize, y: usize) {
        match self {
            Shape::Horizont => {
                board.occupy(y, 0b01111000 >> x);
            }
            Shape::Plus => {
                board.occupy(y, 0b00100000 >> x);
                board.occupy(y + 1, 0b01110000 >> x);
                board.occupy(y + 2, 0b00100000 >> x);
            }
            Shape::Corner => {
                board.occupy(y, 0b01110000 >> x);
                board.occupy(y + 1, 0b00010000 >> x);
                board.occupy(y + 2, 0b00010000 >> x);
            }
            Shape::Line => {
                board.occupy(y, 0b01000000 >> x);
                board.occupy(y + 1, 0b01000000 >> x);
                board.occupy(y + 2, 0b01000000 >> x);
                board.occupy(y + 3, 0b01000000 >> x);
            }
            Shape::Cube => {
                board.occupy(y, 0b01100000 >> x);
                board.occupy(y + 1, 0b01100000 >> x);
            }
        }
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> Result<usize, ParseIntError> {
    let shapes = Shape::all().into_iter().cycle();
    let mut steam = input.trim().as_bytes().iter().map(Steam::from).cycle();

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
