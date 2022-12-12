use std::{
    collections::{HashSet, VecDeque},
    num::ParseIntError,
};

use aoc_runner_derive::aoc;

struct HeightMap<'a> {
    data: &'a [u8],
    end: (usize, usize),
    start: (usize, usize),
    stride: usize,
}

impl<'a> HeightMap<'a> {
    fn new(data: &'a str) -> Self {
        assert!(data.is_ascii());

        let data = data.as_bytes();
        let stride = data.iter().position(|&c| c == b'\n').unwrap() + 1;

        let end = data.iter().position(|&c| c == b'E').unwrap();
        let end = (end % stride, end / stride);

        let start = data.iter().position(|&c| c == b'S').unwrap();
        let start = (start % stride, start / stride);

        HeightMap {
            data,
            end,
            start,
            stride,
        }
    }

    fn width(&self) -> usize {
        self.stride - 1
    }

    fn height(&self) -> usize {
        (self.data.len() + 1) / self.stride
    }

    fn get(&self, (x, y): (usize, usize)) -> u8 {
        assert!(x < self.stride);
        assert!(y * self.stride + x < self.data.len());

        let result = self.data[y * self.stride + x];

        match result {
            b'S' => b'a',
            b'E' => b'z',
            _ => result,
        }
    }

    fn reachable_neighbors(
        &'a self,
        (x, y): (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> {
        let mut result = Vec::new();

        if x > 0 {
            result.push(((x - 1, y), self.get((x - 1, y))));
        }

        if x < self.width() - 1 {
            result.push(((x + 1, y), self.get((x + 1, y))));
        }

        if y > 0 {
            result.push(((x, y - 1), self.get((x, y - 1))));
        }

        if y < self.height() - 1 {
            result.push(((x, y + 1), self.get((x, y + 1))));
        }

        let max_height = self.get((x, y)) + 1;

        result
            .into_iter()
            .filter(move |&(_, height)| height <= max_height)
            .map(|(pos, _)| pos)
    }
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> Result<usize, ParseIntError> {
    let map = HeightMap::new(input);
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((map.start, 0));
    visited.insert(map.start);

    while let Some((pos, cost)) = queue.pop_front() {
        for neighbor in map.reachable_neighbors(pos) {
            if neighbor == map.end {
                return Ok(cost + 1);
            }

            if !visited.contains(&neighbor) {
                queue.push_back((neighbor, cost + 1));
                visited.insert(neighbor);
            }
        }
    }

    panic!("No path found");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        let result = super::part1("Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi");
        assert_eq!(result, Ok(31));
    }
}
