use std::num::ParseIntError;

use aoc_runner_derive::aoc;

mod snafu {
    pub fn parse(input: &str) -> u64 {
        let mut tally = 0i64;
        let mut multiplier = 1i64;

        for c in input.chars().rev() {
            match c {
                '=' => tally -= 2 * multiplier,
                '-' => tally -= multiplier,
                '0' => {}
                '1' => tally += multiplier,
                '2' => tally += 2 * multiplier,
                _ => panic!("Invalid input"),
            }

            multiplier *= 5;
        }

        tally as u64
    }

    pub fn stringify(input: u64) -> String {
        let mut result = Vec::<char>::new();

        let mut rest = input;
        let mut multiplier = 1u64;

        while rest > 0 {
            let next_multiplier = multiplier * 5;
            let part = (rest % next_multiplier) / multiplier;

            rest -= part * multiplier;
            multiplier = next_multiplier;

            match part {
                0 => {
                    result.push('0');
                }
                1 => {
                    result.push('1');
                }
                2 => {
                    result.push('2');
                }
                3 => {
                    result.push('=');
                    rest += multiplier;
                }
                4 => {
                    result.push('-');
                    rest += multiplier;
                }
                _ => unreachable!(),
            }
        }

        result.iter().rev().collect()
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn parse() {
            assert_eq!(super::parse("1"), 1);
            assert_eq!(super::parse("2"), 2);
            assert_eq!(super::parse("1="), 3);
            assert_eq!(super::parse("1-"), 4);
            assert_eq!(super::parse("10"), 5);
            assert_eq!(super::parse("11"), 6);
            assert_eq!(super::parse("12"), 7);
            assert_eq!(super::parse("2="), 8);
            assert_eq!(super::parse("2-"), 9);
            assert_eq!(super::parse("20"), 10);
            assert_eq!(super::parse("1=0"), 15);
            assert_eq!(super::parse("1-0"), 20);
            assert_eq!(super::parse("1=11-2"), 2022);
            assert_eq!(super::parse("1-0---0"), 12345);
            assert_eq!(super::parse("1121-1110-1=0"), 314159265);
        }

        #[test]
        fn stringify() {
            assert_eq!(super::stringify(1), "1");
            assert_eq!(super::stringify(2), "2");
            assert_eq!(super::stringify(3), "1=");
            assert_eq!(super::stringify(4), "1-");
            assert_eq!(super::stringify(5), "10");
            assert_eq!(super::stringify(6), "11");
            assert_eq!(super::stringify(7), "12");
            assert_eq!(super::stringify(8), "2=");
            assert_eq!(super::stringify(9), "2-");
            assert_eq!(super::stringify(10), "20");
            assert_eq!(super::stringify(15), "1=0");
            assert_eq!(super::stringify(20), "1-0");
            assert_eq!(super::stringify(2022), "1=11-2");
            assert_eq!(super::stringify(12345), "1-0---0");
            assert_eq!(super::stringify(314159265), "1121-1110-1=0");
        }
    }
}

#[aoc(day25, part1)]
pub fn part1(input: &str) -> Result<String, ParseIntError> {
    Ok(snafu::stringify(input.lines().map(snafu::parse).sum()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        let result = super::part1(
            "1=-0-2\n12111\n2=0=\n21\n2=01\n111\n20012\n112\n1=-1=\n1-12\n12\n1=\n122",
        );
        assert_eq!(result, Ok(String::from("2=-1=0")));
    }
}
