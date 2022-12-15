use std::{cmp::Ordering, num::ParseIntError, ops::RangeInclusive, str::FromStr};

use aoc_runner_derive::aoc;
use itertools::Itertools;

#[cfg(test)]
const TARGET_Y: i32 = 10;
#[cfg(not(test))]
const TARGET_Y: i32 = 2000000;

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn distance_to(&self, other: &Coord) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl FromStr for Coord {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').unwrap();

        Ok(Coord {
            x: x.trim().strip_prefix("x=").unwrap().parse()?,
            y: y.trim().strip_prefix("y=").unwrap().parse()?,
        })
    }
}

trait MergeRanges {
    fn merge(&mut self);
}

impl MergeRanges for Vec<RangeInclusive<i32>> {
    fn merge(&mut self) {
        assert!(!self.is_empty());

        self.sort_by(|lhs, rhs| match lhs.start().cmp(rhs.start()) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => lhs.end().cmp(rhs.end()),
        });

        let mut idx = 1;

        while idx < self.len() {
            if *self[idx].start() > (self[idx - 1].end() + 1) {
                idx += 1;
                continue;
            }

            let new = RangeInclusive::new(
                *self[idx - 1].start(),
                *self[idx].end().max(self[idx - 1].end()),
            );

            self[idx - 1] = new;
            self.remove(idx);
        }
    }
}

#[derive(Debug)]
struct Sensor {
    closest_beacon: Coord,
    position: Coord,
}

impl FromStr for Sensor {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sensor, beacon) = s.split_once(':').unwrap();

        let sensor = sensor.trim().strip_prefix("Sensor at").unwrap();
        let beacon = beacon.trim().strip_prefix("closest beacon is at").unwrap();

        Ok(Sensor {
            closest_beacon: beacon.parse()?,
            position: sensor.parse()?,
        })
    }
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> Result<usize, ParseIntError> {
    let sensors = input
        .lines()
        .map(|line| line.parse::<Sensor>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut ranges = sensors
        .iter()
        .filter_map(|sensor| {
            let reach = sensor.position.distance_to(&sensor.closest_beacon);
            let dy = sensor.position.y.abs_diff(TARGET_Y);

            if dy > reach {
                return None;
            }

            Some(RangeInclusive::new(
                sensor.position.x.checked_sub_unsigned(reach - dy).unwrap(),
                sensor.position.x.checked_add_unsigned(reach - dy).unwrap(),
            ))
        })
        .collect::<Vec<_>>();

    ranges.merge();

    let beacons_in_target = sensors
        .iter()
        .filter(|sensor| sensor.closest_beacon.y == TARGET_Y)
        .map(|sensor| &sensor.closest_beacon)
        .filter(|beacon| ranges.iter().any(|range| range.contains(&beacon.x)))
        .map(|beacon| beacon.x)
        .unique()
        .count();

    let scanned_positions = ranges
        .iter()
        .map(|range| (range.end() - range.start() + 1) as usize)
        .sum::<usize>();

    Ok(scanned_positions - beacons_in_target)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        let result = super::part1("Sensor at x=2, y=18: closest beacon is at x=-2, y=15\nSensor at x=9, y=16: closest beacon is at x=10, y=16\nSensor at x=13, y=2: closest beacon is at x=15, y=3\nSensor at x=12, y=14: closest beacon is at x=10, y=16\nSensor at x=10, y=20: closest beacon is at x=10, y=16\nSensor at x=14, y=17: closest beacon is at x=10, y=16\nSensor at x=8, y=7: closest beacon is at x=2, y=10\nSensor at x=2, y=0: closest beacon is at x=2, y=10\nSensor at x=0, y=11: closest beacon is at x=2, y=10\nSensor at x=20, y=14: closest beacon is at x=25, y=17\nSensor at x=17, y=20: closest beacon is at x=21, y=22\nSensor at x=16, y=7: closest beacon is at x=15, y=3\nSensor at x=14, y=3: closest beacon is at x=15, y=3\nSensor at x=20, y=1: closest beacon is at x=15, y=3");
        assert_eq!(result, Ok(26));
    }
}
