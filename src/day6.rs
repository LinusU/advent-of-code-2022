use std::collections::{HashSet, VecDeque};

use aoc_runner_derive::aoc;
use itertools::Itertools;

struct MarkerFinder {
    chars: VecDeque<char>,
    result: usize,
    size: usize,
}

impl MarkerFinder {
    fn new(size: usize) -> Self {
        MarkerFinder {
            chars: VecDeque::with_capacity(size),
            result: 0,
            size,
        }
    }

    fn feed(&mut self, c: char) -> Option<usize> {
        if self.chars.len() == self.size {
            self.chars.pop_front();
        }

        self.chars.push_back(c);
        self.result += 1;

        if self.chars.len() != self.size {
            return None;
        }

        let mut seen = HashSet::<char>::new();

        for char in self.chars.iter() {
            if seen.contains(char) {
                return None;
            }

            seen.insert(*char);
        }

        Some(self.result)
    }
}

#[aoc(day6, part1)]
pub fn part1(input: &str) -> usize {
    let pos = input.chars().tuple_windows().find_position(|(a, b, c, d)| {
        if a == b || a == c || a == d {
            return false;
        }

        if b == c || b == d {
            return false;
        }

        if c == d {
            return false;
        }

        true
    });

    pos.unwrap().0 + 4
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let mut finder = MarkerFinder::new(14);

    for c in input.chars() {
        if let Some(result) = finder.feed(c) {
            return result;
        }
    }

    panic!("No result found")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        let result = super::part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(result, 7);
    }

    #[test]
    fn test_case_2() {
        let result = super::part1("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(result, 5);
    }

    #[test]
    fn test_case_3() {
        let result = super::part1("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(result, 6);
    }

    #[test]
    fn test_case_4() {
        let result = super::part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(result, 10);
    }

    #[test]
    fn test_case_5() {
        let result = super::part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(result, 11);
    }

    #[test]
    fn test_case_6() {
        let result = super::part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(result, 19);
    }

    #[test]
    fn test_case_7() {
        let result = super::part2("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(result, 23);
    }

    #[test]
    fn test_case_8() {
        let result = super::part2("nppdvjthqldpwncqszvftbrmjlhg");
        assert_eq!(result, 23);
    }

    #[test]
    fn test_case_9() {
        let result = super::part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        assert_eq!(result, 29);
    }

    #[test]
    fn test_case_10() {
        let result = super::part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
        assert_eq!(result, 26);
    }
}
