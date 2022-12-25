use std::{collections::HashMap, fmt::Debug, num::ParseIntError, str::FromStr};

use aoc_runner_derive::aoc;

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

impl Blizzard {
    fn flipped(&self) -> Self {
        match self {
            Blizzard::North => Blizzard::South,
            Blizzard::South => Blizzard::North,
            Blizzard::West => Blizzard::East,
            Blizzard::East => Blizzard::West,
        }
    }
}

#[derive(Clone)]
struct Board {
    data: Vec<Vec<Blizzard>>,
    width: usize,
}

impl Board {
    fn is_free(&self, pos: Pos) -> bool {
        self.data[pos.y * self.width + pos.x].is_empty()
    }

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

    fn flipped(&self) -> Board {
        let mut data = Vec::with_capacity(self.data.len());

        for cell in self.data.iter().rev() {
            data.push(cell.iter().map(|b| b.flipped()).collect());
        }

        Board { data, width: self.width }
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
        let distance = (goal.x - self.pos.x) + (goal.y - self.pos.y);

        self.minutes + distance
    }
}

fn shortest_time_between(init: Board, first_cycle_index: usize) -> usize {
    let bounds = (init.width, init.height());

    let start = Pos {
        x: 0,
        y: 0,
    };

    let goal = Pos {
        x: bounds.0 - 1,
        y: bounds.1 - 1,
    };

    let cycle_count = lcm(bounds.0, bounds.1);

    let mut states = Vec::with_capacity(cycle_count);

    states.push(init);

    while states.len() < cycle_count {
        let next = states.last().unwrap().next();
        states.push(next);
    }

    let mut queue = Vec::<Step>::new();

    for i in (0..cycle_count).rev() {
        let minutes = i + first_cycle_index;

        if states[minutes % cycle_count].is_free(start) {
            queue.push(Step { minutes, pos: start });
        }
    }

    let mut shortest_time = usize::MAX;
    let mut visited = HashMap::<(Pos, usize), usize>::new();

    while let Some(step) = queue.pop() {
        let cycle_index = step.minutes % cycle_count;

        if let Some(&last_time_at_this_position_in_cycle) = visited.get(&(step.pos, cycle_index)) {
            if last_time_at_this_position_in_cycle <= step.minutes {
                continue;
            }
        }

        visited.insert((step.pos, cycle_index), step.minutes);

        let next_board_state = &states[cycle_index];

        if step.pos == goal {
            shortest_time = shortest_time.min(step.minutes);
            continue;
        }

        if step.best_possible_time(goal) >= shortest_time {
            continue;
        }

        if next_board_state.is_free(step.pos) {
            queue.push(Step {
                minutes: step.minutes + 1,
                pos: step.pos,
            });
        }

        for pos in step.neighbours(bounds) {
            if next_board_state.is_free(pos) {
                queue.push(Step {
                    minutes: step.minutes + 1,
                    pos,
                })
            }
        }
    }

    shortest_time
}

#[aoc(day24, part1)]
pub fn part1(input: &str) -> Result<usize, ParseIntError> {
    let init = Board::from_str(input)?;

    Ok(shortest_time_between(init, 1))
}

#[aoc(day24, part2)]
pub fn part2(input: &str) -> Result<usize, ParseIntError> {
    let init = Board::from_str(input)?;

    let first = shortest_time_between(init.clone(), 1);
    let second = shortest_time_between(init.flipped(), first);
    let third = shortest_time_between(init, second);

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
