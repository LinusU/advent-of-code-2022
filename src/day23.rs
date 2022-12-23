use std::{
    collections::{HashMap, HashSet},
    num::ParseIntError,
    ops::RangeInclusive,
    str::FromStr,
};

use aoc_runner_derive::aoc;

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos {
            x: x as i32,
            y: y as i32,
        }
    }

    fn delta(&self, dx: i32, dy: i32) -> Pos {
        Pos {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    fn go(&self, dir: Direction) -> Pos {
        match dir {
            Direction::North => self.delta(0, -1),
            Direction::South => self.delta(0, 1),
            Direction::West => self.delta(-1, 0),
            Direction::East => self.delta(1, 0),
        }
    }

    fn look(&self, dir: Direction) -> [Pos; 3] {
        match dir {
            Direction::North => [self.delta(-1, -1), self.delta(0, -1), self.delta(1, -1)],
            Direction::South => [self.delta(-1, 1), self.delta(0, 1), self.delta(1, 1)],
            Direction::West => [self.delta(-1, -1), self.delta(-1, 0), self.delta(-1, 1)],
            Direction::East => [self.delta(1, -1), self.delta(1, 0), self.delta(1, 1)],
        }
    }
}

struct Board(HashSet<Pos>);

impl Board {
    fn bounds(&self) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
        let min_x = self.0.iter().map(|elf| elf.x).min().unwrap();
        let max_x = self.0.iter().map(|elf| elf.x).max().unwrap();
        let min_y = self.0.iter().map(|elf| elf.y).min().unwrap();
        let max_y = self.0.iter().map(|elf| elf.y).max().unwrap();

        (
            RangeInclusive::new(min_x, max_x),
            RangeInclusive::new(min_y, max_y),
        )
    }

    fn size(&self) -> usize {
        let (x, y) = self.bounds();

        (x.end() - x.start() + 1) as usize * (y.end() - y.start() + 1) as usize
    }

    fn iter(&self) -> impl Iterator<Item = &Pos> {
        self.0.iter()
    }

    fn elves(&self) -> usize {
        self.0.len()
    }

    fn has_elf(&self, pos: &Pos) -> bool {
        self.0.contains(pos)
    }

    fn has_adjecent(&self, pos: &Pos) -> bool {
        self.has_elf(&pos.delta(-1, -1))
            || self.has_elf(&pos.delta(0, -1))
            || self.has_elf(&pos.delta(1, -1))
            || self.has_elf(&pos.delta(-1, 0))
            || self.has_elf(&pos.delta(1, 0))
            || self.has_elf(&pos.delta(-1, 1))
            || self.has_elf(&pos.delta(0, 1))
            || self.has_elf(&pos.delta(1, 1))
    }

    fn look(&self, pos: &Pos, dir: Direction) -> usize {
        pos.look(dir).iter().filter(|pos| self.has_elf(pos)).count()
    }
}

impl FromStr for Board {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Board(
            s.lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars().enumerate().flat_map(move |(x, c)| {
                        if c == '#' {
                            Some(Pos::new(x, y))
                        } else {
                            None
                        }
                    })
                })
                .collect::<HashSet<_>>(),
        ))
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (xs, ys) = self.bounds();

        writeln!(f)?;

        for y in ys {
            for x in xs.clone() {
                if self.has_elf(&Pos { x, y }) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[aoc(day23, part1)]
pub fn part1(input: &str) -> Result<usize, ParseIntError> {
    let mut board = Board::from_str(input)?;

    let directions = vec![
        vec![
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ],
        vec![
            Direction::South,
            Direction::West,
            Direction::East,
            Direction::North,
        ],
        vec![
            Direction::West,
            Direction::East,
            Direction::North,
            Direction::South,
        ],
        vec![
            Direction::East,
            Direction::North,
            Direction::South,
            Direction::West,
        ],
    ];

    for directions in directions.iter().cycle().take(10) {
        let mut proposed = HashMap::<Pos, usize>::new();

        for &elf in board.iter() {
            if board.has_adjecent(&elf) {
                for &dir in directions {
                    if board.look(&elf, dir) == 0 {
                        *proposed.entry(elf.go(dir)).or_insert(0) += 1;
                        break;
                    }
                }
            }
        }

        let mut next = HashSet::<Pos>::new();

        'elf: for &elf in board.iter() {
            if board.has_adjecent(&elf) {
                for &dir in directions {
                    if board.look(&elf, dir) == 0 {
                        if *proposed.get(&elf.go(dir)).unwrap() == 1 {
                            next.insert(elf.go(dir));
                            continue 'elf;
                        }

                        break;
                    }
                }
            }

            next.insert(elf);
        }

        board = Board(next);
    }

    Ok(board.size() - board.elves())
}

#[aoc(day23, part2)]
pub fn part2(input: &str) -> Result<usize, ParseIntError> {
    let mut board = Board::from_str(input)?;

    let directions = vec![
        vec![
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ],
        vec![
            Direction::South,
            Direction::West,
            Direction::East,
            Direction::North,
        ],
        vec![
            Direction::West,
            Direction::East,
            Direction::North,
            Direction::South,
        ],
        vec![
            Direction::East,
            Direction::North,
            Direction::South,
            Direction::West,
        ],
    ];

    for (idx, directions) in directions.iter().cycle().enumerate() {
        let mut proposed = HashMap::<Pos, usize>::new();

        for &elf in board.iter() {
            if board.has_adjecent(&elf) {
                for &dir in directions {
                    if board.look(&elf, dir) == 0 {
                        *proposed.entry(elf.go(dir)).or_insert(0) += 1;
                        break;
                    }
                }
            }
        }

        let mut next = HashSet::<Pos>::new();
        let mut done = true;

        'elf: for &elf in board.iter() {
            if board.has_adjecent(&elf) {
                done = false;

                for &dir in directions {
                    if board.look(&elf, dir) == 0 {
                        if *proposed.get(&elf.go(dir)).unwrap() == 1 {
                            next.insert(elf.go(dir));
                            continue 'elf;
                        }

                        break;
                    }
                }
            }

            next.insert(elf);
        }

        if done {
            return Ok(idx + 1);
        }

        board = Board(next);
    }

    panic!("Endless iterator ended");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        let result = super::part1("....#..\n..###.#\n#...#.#\n.#...##\n#.###..\n##.#.##\n.#..#..");
        assert_eq!(result, Ok(110));
    }

    #[test]
    fn test_case_2() {
        let result = super::part2("....#..\n..###.#\n#...#.#\n.#...##\n#.###..\n##.#.##\n.#..#..");
        assert_eq!(result, Ok(20));
    }
}
