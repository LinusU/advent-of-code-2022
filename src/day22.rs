use std::{iter::from_generator, num::ParseIntError, str::FromStr};

use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map, multi::many1,
    IResult,
};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Direction {
    East = 0,
    South = 1,
    West = 2,
    North = 3,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
            Direction::North => Direction::West,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            Direction::North => Direction::East,
        }
    }

    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
            Direction::North => (0, -1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Void,
    Floor,
    Wall,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            ' ' => Cell::Void,
            '.' => Cell::Floor,
            '#' => Cell::Wall,
            _ => panic!("Invalid input"),
        }
    }
}

struct Map {
    data: Vec<Cell>,
    width: usize,
}

impl Map {
    fn start(&self) -> (usize, usize) {
        let position = self
            .data
            .iter()
            .position(|&cell| cell == Cell::Floor)
            .unwrap();

        (position % self.width, position / self.width)
    }

    fn get(&self, pos: (usize, usize)) -> Cell {
        self.data[pos.1 * self.width + pos.0]
    }

    fn height(&self) -> usize {
        self.data.len() / self.width
    }

    fn wrapping_add_2d(&self, pos: (usize, usize), delta: (isize, isize)) -> (usize, usize) {
        (
            pos.0.checked_add_signed(delta.0).unwrap_or(self.width - 1) % self.width,
            pos.1
                .checked_add_signed(delta.1)
                .unwrap_or(self.height() - 1)
                % self.height(),
        )
    }

    //   ABBH
    //   CDDF
    //   CD
    //   EF
    // CEEF
    // AGGH
    // AG
    // BH
    fn wrapping_add_3d(&self, pos: (usize, usize), dir: Direction) -> ((usize, usize), Direction) {
        match (dir, pos.0, pos.1) {
            // H - F
            (Direction::East, 149, 0..50) => ((99, 149 - pos.1), Direction::West),
            // D - F
            (Direction::East, 99, 50..100) => ((100 + (pos.1 - 50), 49), Direction::North),
            // F - H
            (Direction::East, 99, 100..150) => ((149, 49 - (pos.1 - 100)), Direction::West),
            // G - H
            (Direction::East, 49, 150..200) => ((50 + (pos.1 - 150), 149), Direction::North),
            // D - F
            (Direction::South, 100..150, 49) => ((99, 50 + (pos.0 - 100)), Direction::West),
            // G - H
            (Direction::South, 50..100, 149) => ((49, 150 + (pos.0 - 50)), Direction::West),
            // B - H
            (Direction::South, 0..50, 199) => ((100 + pos.0, 0), Direction::South),
            // A - C
            (Direction::West, 50, 0..50) => ((0, 149 - pos.1), Direction::East),
            // C - E
            (Direction::West, 50, 50..100) => ((pos.1 - 50, 100), Direction::South),
            // C - A
            (Direction::West, 0, 100..150) => ((50, 49 - (pos.1 - 100)), Direction::East),
            // A - B
            (Direction::West, 0, 150..200) => ((50 + (pos.1 - 150), 0), Direction::South),
            // A - B
            (Direction::North, 50..100, 0) => ((0, 150 + (pos.0 - 50)), Direction::East),
            // B - H
            (Direction::North, 100..150, 0) => ((pos.0 - 100, 199), Direction::North),
            // C - E
            (Direction::North, 0..50, 100) => ((50, 50 + pos.0), Direction::East),
            // inside
            _ => (self.wrapping_add_2d(pos, dir.delta()), dir),
        }
    }
}

impl FromStr for Map {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().map(|l| l.len()).max().unwrap();

        let data = s
            .lines()
            .flat_map(|line| {
                from_generator(|| {
                    let void = width - line.len();

                    for char in line.chars() {
                        yield char.into();
                    }

                    for _ in 0..void {
                        yield Cell::Void;
                    }
                })
            })
            .collect::<Vec<_>>();

        Ok(Map { data, width })
    }
}

enum Instruction {
    Forward(usize),
    TurnLeft,
    TurnRight,
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Instruction> {
        alt((
            map(tag("L"), |_| Self::TurnLeft),
            map(tag("R"), |_| Self::TurnRight),
            map(digit1, |s: &str| Self::Forward(s.parse().unwrap())),
        ))(input)
    }

    fn parse_list(input: &str) -> IResult<&str, Vec<Instruction>> {
        many1(Instruction::parse)(input)
    }
}

#[aoc(day22, part1)]
pub fn part1(input: &str) -> Result<usize, ParseIntError> {
    let (map, instructions) = input.split_once("\n\n").unwrap();

    let map = Map::from_str(map)?;
    let instructions = Instruction::parse_list(instructions).unwrap().1;

    let mut pos = map.start();
    let mut dir = Direction::East;

    for instruction in instructions {
        let distance = match instruction {
            Instruction::TurnLeft => {
                dir = dir.turn_left();
                continue;
            }
            Instruction::TurnRight => {
                dir = dir.turn_right();
                continue;
            }
            Instruction::Forward(distance) => distance,
        };

        let delta = dir.delta();

        for _ in 0..distance {
            let mut next = map.wrapping_add_2d(pos, delta);

            let cell = map.get(next);

            match cell {
                Cell::Floor => {
                    pos = next;
                }
                Cell::Wall => {
                    break;
                }
                Cell::Void => loop {
                    next = map.wrapping_add_2d(next, delta);

                    match map.get(next) {
                        Cell::Floor => {
                            pos = next;
                            break;
                        }
                        Cell::Wall => {
                            break;
                        }
                        Cell::Void => {}
                    }
                },
            }
        }
    }

    Ok((pos.1 + 1) * 1000 + (pos.0 + 1) * 4 + (dir as u8 as usize))
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> Result<usize, ParseIntError> {
    let (map, instructions) = input.split_once("\n\n").unwrap();

    let map = Map::from_str(map)?;
    let instructions = Instruction::parse_list(instructions).unwrap().1;

    let mut pos = map.start();
    let mut dir = Direction::East;

    for instruction in instructions {
        let distance = match instruction {
            Instruction::TurnLeft => {
                dir = dir.turn_left();
                continue;
            }
            Instruction::TurnRight => {
                dir = dir.turn_right();
                continue;
            }
            Instruction::Forward(distance) => distance,
        };

        for _ in 0..distance {
            let (next_pos, next_dir) = map.wrapping_add_3d(pos, dir);

            let cell = map.get(next_pos);

            match cell {
                Cell::Floor => {
                    pos = next_pos;
                    dir = next_dir;
                }
                Cell::Wall => {
                    break;
                }
                Cell::Void => {
                    panic!("Stepped into the void {next_pos:?} from {pos:?} (dir={dir:?})")
                }
            }
        }
    }

    Ok((pos.1 + 1) * 1000 + (pos.0 + 1) * 4 + (dir as u8 as usize))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        let result = super::part1("        ...#\n        .#..\n        #...\n        ....\n...#.......#\n........#...\n..#....#....\n..........#.\n        ...#....\n        .....#..\n        .#......\n        ......#.\n\n10R5L5R10L4R5L5");
        assert_eq!(result, Ok(6032));
    }
}
