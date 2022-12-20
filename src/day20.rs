use std::{
    num::ParseIntError,
    ops::{Index, Range},
    str::FromStr,
};

use aoc_runner_derive::aoc;

struct Number {
    index: u16,
    value: i16,
}

impl Number {
    fn is_zero(&self) -> bool {
        self.value == 0
    }

    fn distance(&self, decryption_key: usize, wrap: usize) -> usize {
        ((self.value.unsigned_abs() as usize) * decryption_key) % wrap
    }

    fn signum(&self) -> i16 {
        self.value.signum()
    }

    fn value(&self, decryption_key: usize) -> i64 {
        (self.value as i64) * (decryption_key as i64)
    }
}

impl TryFrom<(usize, &str)> for Number {
    type Error = ParseIntError;

    fn try_from(value: (usize, &str)) -> Result<Self, Self::Error> {
        Ok(Self {
            index: value.0 as u16,
            value: value.1.parse()?,
        })
    }
}

struct File(Vec<Number>);

impl File {
    fn pos(&self, idx: u16) -> usize {
        self.0.iter().position(|n| n.index == idx).unwrap()
    }

    fn last_pos(&self) -> usize {
        self.0.len() - 1
    }

    fn range(&self) -> Range<u16> {
        0..(self.0.len() as u16)
    }

    fn swap(&mut self, lhs: usize, rhs: usize) {
        self.0.swap(lhs, rhs)
    }

    fn grove_coordinates(&self, decryption_key: usize) -> i64 {
        let start = self.0.iter().position(|n| n.is_zero()).unwrap();

        self.0[(start + 1000) % self.0.len()].value(decryption_key)
            + self.0[(start + 2000) % self.0.len()].value(decryption_key)
            + self.0[(start + 3000) % self.0.len()].value(decryption_key)
    }
}

impl FromStr for File {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .lines()
            .enumerate()
            .map(Number::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self(numbers))
    }
}

impl Index<usize> for File {
    type Output = Number;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> Result<i64, ParseIntError> {
    const DECRYPTION_KEY: usize = 1;

    let mut file = File::from_str(input)?;
    let last_pos = file.last_pos();

    for idx in file.range() {
        let mut pos = file.pos(idx);
        let number = &file[pos];

        if number.is_zero() {
            continue;
        }

        let direction = number.signum();
        let distance = number.distance(DECRYPTION_KEY, last_pos);

        for _ in 0..distance {
            match direction {
                -1 => {
                    if pos == 0 {
                        file.swap(pos, last_pos);
                        pos = last_pos;
                    } else {
                        file.swap(pos, pos - 1);
                        pos -= 1;
                    }
                }
                1 => {
                    if pos == last_pos {
                        file.swap(pos, 0);
                        pos = 0;
                    } else {
                        file.swap(pos, pos + 1);
                        pos += 1;
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    Ok(file.grove_coordinates(DECRYPTION_KEY))
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> Result<i64, ParseIntError> {
    const DECRYPTION_KEY: usize = 811589153;

    let mut file = File::from_str(input)?;
    let last_pos = file.last_pos();

    for _ in 0..10 {
        for idx in file.range() {
            let mut pos = file.pos(idx);
            let number = &file[pos];

            if number.is_zero() {
                continue;
            }

            let direction = number.signum();
            let distance = number.distance(DECRYPTION_KEY, last_pos);

            for _ in 0..distance {
                match direction {
                    -1 => {
                        if pos == 0 {
                            file.swap(pos, last_pos);
                            pos = last_pos;
                        } else {
                            file.swap(pos, pos - 1);
                            pos -= 1;
                        }
                    }
                    1 => {
                        if pos == last_pos {
                            file.swap(pos, 0);
                            pos = 0;
                        } else {
                            file.swap(pos, pos + 1);
                            pos += 1;
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    Ok(file.grove_coordinates(DECRYPTION_KEY))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        let result = super::part1("1\n2\n-3\n3\n-2\n0\n4");
        assert_eq!(result, Ok(3));
    }

    #[test]
    fn test_case_2() {
        let result = super::part2("1\n2\n-3\n3\n-2\n0\n4");
        assert_eq!(result, Ok(1623178306));
    }
}
