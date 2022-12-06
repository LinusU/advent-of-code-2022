use aoc_runner_derive::aoc;
use itertools::Itertools;

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
}
