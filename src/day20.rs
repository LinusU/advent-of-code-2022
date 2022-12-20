use std::num::ParseIntError;

use aoc_runner_derive::aoc;

struct Number {
    index: u16,
    value: i16,
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> Result<i16, ParseIntError> {
    let mut numbers = input
        .lines()
        .enumerate()
        .map(|(idx, line)| {
            line.parse::<i16>().map(|value| Number {
                index: idx as u16,
                value,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let last_pos = numbers.len() - 1;

    for idx in 0..(numbers.len() as u16) {
        let mut pos = numbers.iter().position(|n| n.index == idx).unwrap();
        let value = numbers[pos].value;

        if value == 0 {
            continue;
        }

        let direction = value.signum();
        let distance = value.abs();

        for _ in 0..distance {
            match direction {
                -1 => {
                    if pos == 0 {
                        numbers.swap(pos, last_pos);
                        pos = last_pos;
                    } else {
                        numbers.swap(pos, pos - 1);
                        pos -= 1;
                    }
                }
                1 => {
                    if pos == last_pos {
                        numbers.swap(pos, 0);
                        pos = 0;
                    } else {
                        numbers.swap(pos, pos + 1);
                        pos += 1;
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    let start = numbers.iter().position(|n| n.value == 0).unwrap();

    Ok(numbers[(start + 1000) % numbers.len()].value
        + numbers[(start + 2000) % numbers.len()].value
        + numbers[(start + 3000) % numbers.len()].value)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        let result = super::part1("1\n2\n-3\n3\n-2\n0\n4");
        assert_eq!(result, Ok(3));
    }
}
