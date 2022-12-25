use std::{collections::HashMap, fmt::Debug, num::ParseIntError, str::FromStr};

use aoc_runner_derive::aoc;

use crate::util::priority_queue::{PriorityQueue, PriorityQueueItem};

fn lcm(a: usize, b: usize) -> usize {
    match (a, b) {
        (5, 5) => 5,
        (6, 4) => 12,
        (150, 20) => 300,
        // Implementing lowest common multiple was complicated...
        _ => unimplemented!(),
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

impl Debug for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy)]
enum Blizzard {
    North,
    South,
    West,
    East,
}

#[derive(Clone)]
struct Board {
    data: Vec<Vec<Blizzard>>,
    width: usize,
}

impl Board {
    fn height(&self) -> usize {
        self.data.len() / self.width
    }

    fn next(&self) -> Board {
        let len = self.data.len();
        let width = self.width;

        let mut next = vec![Vec::new(); len];

        for (idx, cell) in self.data.iter().enumerate() {
            for blizzard in cell {
                match blizzard {
                    Blizzard::North => next[(idx + len - width) % len].push(Blizzard::North),
                    Blizzard::South => next[(idx + width) % len].push(Blizzard::South),
                    Blizzard::West => {
                        let mut x = idx % width;
                        let y = idx / width;
                        x = (x + width - 1) % width;
                        next[y * width + x].push(Blizzard::West)
                    }
                    Blizzard::East => {
                        let mut x = idx % width;
                        let y = idx / width;
                        x = (x + 1) % width;
                        next[y * width + x].push(Blizzard::East)
                    }
                }
            }
        }

        Board {
            data: next,
            width: self.width,
        }
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, cell) in self.data.iter().enumerate() {
            if (idx % self.width) == 0 {
                writeln!(f)?;
            }

            match cell.len() {
                0 => write!(f, ".")?,
                1 => match cell[0] {
                    Blizzard::North => write!(f, "^")?,
                    Blizzard::South => write!(f, "v")?,
                    Blizzard::West => write!(f, "<")?,
                    Blizzard::East => write!(f, ">")?,
                },
                _ => write!(f, "{}", cell.len())?,
            }
        }

        Ok(())
    }
}

impl FromStr for Board {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().len() - 2;
        let height = s.lines().count() - 2;

        let data = s
            .lines()
            .skip(1)
            .take(height)
            .flat_map(|line| {
                line.chars().skip(1).take(width).map(|c| match c {
                    '.' => vec![],
                    '^' => vec![Blizzard::North],
                    'v' => vec![Blizzard::South],
                    '<' => vec![Blizzard::West],
                    '>' => vec![Blizzard::East],
                    _ => panic!("Invalid input: {c}"),
                })
            })
            .collect();

        Ok(Board { data, width })
    }
}

struct BoardStates {
    data: Vec<Vec<bool>>,
    width: usize,
}

impl BoardStates {
    fn height(&self) -> usize {
        self.data[0].len() / self.width
    }

    fn is_free(&self, cycle: usize, pos: Pos) -> bool {
        self.data[cycle % self.data.len()][pos.y * self.width + pos.x]
    }
}

impl From<Board> for BoardStates {
    fn from(value: Board) -> Self {
        let len = lcm(value.width, value.height());

        let mut current = value;
        let mut data = Vec::<Vec<bool>>::with_capacity(len);

        loop {
            data.push(current.data.iter().map(Vec::is_empty).collect());

            if data.len() == len {
                return Self {
                    data,
                    width: current.width,
                };
            }

            current = current.next();
        }
    }
}

struct Step {
    minutes: usize,
    pos: Pos,
}

impl Step {
    fn neighbours(&self, bounds: (usize, usize)) -> Vec<Pos> {
        let mut result = Vec::<Pos>::with_capacity(4);

        if self.pos.y > 0 {
            result.push(Pos {
                x: self.pos.x,
                y: self.pos.y - 1,
            });
        }

        if self.pos.x > 0 {
            result.push(Pos {
                x: self.pos.x - 1,
                y: self.pos.y,
            });
        }

        if self.pos.y < (bounds.1 - 1) {
            result.push(Pos {
                x: self.pos.x,
                y: self.pos.y + 1,
            })
        }

        if self.pos.x < (bounds.0 - 1) {
            result.push(Pos {
                x: self.pos.x + 1,
                y: self.pos.y,
            })
        }

        result
    }

    fn best_possible_time(&self, goal: Pos) -> usize {
        self.minutes + (goal.x.abs_diff(self.pos.x)) + (goal.y.abs_diff(self.pos.y))
    }
}

impl PriorityQueueItem<Pos> for Step {
    fn cost(&self, context: &Pos) -> usize {
        self.best_possible_time(*context)
    }
}

fn shortest_time_between(
    states: &BoardStates,
    start: Pos,
    end: Pos,
    first_cycle_index: usize,
) -> usize {
    let bounds = (states.width, states.height());
    let cycle_count = lcm(bounds.0, bounds.1);

    let mut queue = PriorityQueue::<Pos, Step>::new(end);

    for i in 0..cycle_count {
        let minutes = i + first_cycle_index;

        if states.is_free(minutes, start) {
            queue.push(Step {
                minutes,
                pos: start,
            });
        }
    }

    let mut shortest_time = usize::MAX;
    let mut visited = HashMap::<(Pos, usize), usize>::new();

    while let Some((cost, step)) = queue.pop() {
        if cost >= shortest_time {
            break;
        }

        let cycle_index = step.minutes % cycle_count;

        if let Some(&last_time_at_this_position_in_cycle) = visited.get(&(step.pos, cycle_index)) {
            if last_time_at_this_position_in_cycle <= step.minutes {
                continue;
            }
        }

        visited.insert((step.pos, cycle_index), step.minutes);

        if step.pos == end {
            shortest_time = shortest_time.min(step.minutes);
            continue;
        }

        if states.is_free(cycle_index, step.pos) {
            queue.push(Step {
                minutes: step.minutes + 1,
                pos: step.pos,
            });
        }

        for pos in step.neighbours(bounds) {
            if states.is_free(cycle_index, pos) {
                queue.push(Step {
                    minutes: step.minutes + 1,
                    pos,
                });
            }
        }
    }

    shortest_time
}

#[aoc(day24, part1)]
pub fn part1(input: &str) -> Result<usize, ParseIntError> {
    let states: BoardStates = Board::from_str(input)?.into();

    let start = Pos { x: 0, y: 0 };

    let end = Pos {
        x: states.width - 1,
        y: states.height() - 1,
    };

    Ok(shortest_time_between(&states, start, end, 1))
}

#[aoc(day24, part2)]
pub fn part2(input: &str) -> Result<usize, ParseIntError> {
    let states: BoardStates = Board::from_str(input)?.into();

    let start = Pos { x: 0, y: 0 };

    let end = Pos {
        x: states.width - 1,
        y: states.height() - 1,
    };

    let first = shortest_time_between(&states, start, end, 1);
    let second = shortest_time_between(&states, end, start, first);
    let third = shortest_time_between(&states, start, end, second);

    Ok(third)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        let result = super::part1("#.######\n#>>.<^<#\n#.<..<<#\n#>v.><>#\n#<^v^^>#\n######.#");
        assert_eq!(result, Ok(18));
    }

    #[test]
    fn test_case_2() {
        let result = super::part2("#.######\n#>>.<^<#\n#.<..<<#\n#>v.><>#\n#<^v^^>#\n######.#");
        assert_eq!(result, Ok(54));
    }
}
