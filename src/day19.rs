use std::num::ParseIntError;

use aoc_runner_derive::aoc;
use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::map_res, sequence::tuple,
};
use rayon::prelude::*;

fn geode_robot_every_minute(time_left: u8) -> usize {
    (time_left as usize) * (time_left as usize - 1) / 2
}

#[derive(Debug)]
struct Cost {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    ore_robot_cost: Cost,
    clay_robot_cost: Cost,
    obsidian_robot_cost: Cost,
    geode_robot_cost: Cost,
}

impl Blueprint {
    fn parse(s: &str) -> Result<Self, nom::Err<nom::error::Error<&str>>> {
        let (rest, data) = tuple((
            tag("Blueprint "),
            map_res(digit1, str::parse::<usize>),
            tag(": Each ore robot costs "),
            map_res(digit1, str::parse::<usize>),
            tag(" ore. Each clay robot costs "),
            map_res(digit1, str::parse::<usize>),
            tag(" ore. Each obsidian robot costs "),
            map_res(digit1, str::parse::<usize>),
            tag(" ore and "),
            map_res(digit1, str::parse::<usize>),
            tag(" clay. Each geode robot costs "),
            map_res(digit1, str::parse::<usize>),
            tag(" ore and "),
            map_res(digit1, str::parse::<usize>),
            tag(" obsidian."),
        ))(s)?;

        assert_eq!(rest, "");

        Ok(Self {
            id: data.1,
            ore_robot_cost: Cost {
                ore: data.3,
                clay: 0,
                obsidian: 0,
            },
            clay_robot_cost: Cost {
                ore: data.5,
                clay: 0,
                obsidian: 0,
            },
            obsidian_robot_cost: Cost {
                ore: data.7,
                clay: data.9,
                obsidian: 0,
            },
            geode_robot_cost: Cost {
                ore: data.11,
                clay: 0,
                obsidian: data.13,
            },
        })
    }

    fn max_geodes(&self, time: u8) -> usize {
        let mut result = 0usize;
        let mut queue = vec![Step::new(time)];

        while let Some(step) = queue.pop() {
            if step.time_left == 0 {
                result = result.max(step.geodes);
                continue;
            }

            if step.can_afford_always(&self.geode_robot_cost) {
                let score = if step.can_afford(&self.geode_robot_cost) {
                    step.score() + geode_robot_every_minute(step.time_left)
                } else if step.time_left > 1 {
                    step.score() + geode_robot_every_minute(step.time_left - 1)
                } else {
                    step.score()
                };

                result = result.max(score);
                continue;
            }

            if step.best(&self.geode_robot_cost) < result {
                continue;
            }

            queue.push(step.buy_nothing());

            if step.can_afford(&self.geode_robot_cost) {
                queue.push(step.buy_robot(Resource::Geode, &self.geode_robot_cost));
            }

            if step.can_afford(&self.obsidian_robot_cost) {
                queue.push(step.buy_robot(Resource::Obsidian, &self.obsidian_robot_cost));
            }

            if step.can_afford(&self.clay_robot_cost) {
                queue.push(step.buy_robot(Resource::Clay, &self.clay_robot_cost));
            }

            if step.can_afford(&self.ore_robot_cost) {
                queue.push(step.buy_robot(Resource::Ore, &self.ore_robot_cost));
            }
        }

        result
    }
}

struct Step {
    time_left: u8,
    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
    ore_robots: usize,
    clay_robots: usize,
    obsidian_robots: usize,
    geode_robots: usize,
}

impl Step {
    fn new(time_left: u8) -> Self {
        Self {
            time_left,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        }
    }

    fn can_afford(&self, cost: &Cost) -> bool {
        self.ore >= cost.ore && self.clay >= cost.clay && self.obsidian >= cost.obsidian
    }

    fn can_afford_next(&self, cost: &Cost) -> bool {
        (self.ore + self.ore_robots) >= cost.ore
            && (self.clay + self.clay_robots) >= cost.clay
            && (self.obsidian + self.obsidian_robots) >= cost.obsidian
    }

    fn can_afford_always(&self, cost: &Cost) -> bool {
        self.ore_robots >= cost.ore
            && self.clay_robots >= cost.clay
            && self.obsidian_robots >= cost.obsidian
    }

    fn score(&self) -> usize {
        self.geodes + (self.geode_robots * (self.time_left as usize))
    }

    fn best(&self, geode_robot_cost: &Cost) -> usize {
        if self.time_left > 1 && self.can_afford(geode_robot_cost) {
            return self.score() + geode_robot_every_minute(self.time_left);
        }

        if self.time_left > 2 && self.can_afford_next(geode_robot_cost) {
            return self.score() + geode_robot_every_minute(self.time_left - 1);
        }

        if self.time_left > 3 {
            return self.score() + geode_robot_every_minute(self.time_left - 2);
        }

        self.score()
    }

    fn buy_robot(&self, robot_type: Resource, cost: &Cost) -> Self {
        Self {
            time_left: self.time_left - 1,
            ore: self.ore - cost.ore + self.ore_robots,
            clay: self.clay - cost.clay + self.clay_robots,
            obsidian: self.obsidian - cost.obsidian + self.obsidian_robots,
            geodes: self.geodes + self.geode_robots,
            ore_robots: self.ore_robots + (robot_type == Resource::Ore) as usize,
            clay_robots: self.clay_robots + (robot_type == Resource::Clay) as usize,
            obsidian_robots: self.obsidian_robots + (robot_type == Resource::Obsidian) as usize,
            geode_robots: self.geode_robots + (robot_type == Resource::Geode) as usize,
        }
    }

    fn buy_nothing(&self) -> Self {
        Self {
            time_left: self.time_left - 1,
            ore: self.ore + self.ore_robots,
            clay: self.clay + self.clay_robots,
            obsidian: self.obsidian + self.obsidian_robots,
            geodes: self.geodes + self.geode_robots,
            ore_robots: self.ore_robots,
            clay_robots: self.clay_robots,
            obsidian_robots: self.obsidian_robots,
            geode_robots: self.geode_robots,
        }
    }
}

#[aoc(day19, part1)]
pub fn part1(input: &str) -> Result<usize, ParseIntError> {
    let blueprints = input
        .lines()
        .map(Blueprint::parse)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    Ok(blueprints
        .par_iter()
        .map(|blueprint| blueprint.id * blueprint.max_geodes(24))
        .sum())
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> Result<usize, ParseIntError> {
    let blueprints = input
        .lines()
        .take(3)
        .map(Blueprint::parse)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    Ok(blueprints
        .par_iter()
        .map(|blueprint| blueprint.max_geodes(32))
        .product())
}

#[cfg(test)]
mod tests {
    #[test]
    fn geode_robot_every_minute() {
        assert_eq!(super::geode_robot_every_minute(1), 0);
        assert_eq!(super::geode_robot_every_minute(2), 1);
        assert_eq!(super::geode_robot_every_minute(3), 3);
        assert_eq!(super::geode_robot_every_minute(4), 6);
    }

    #[test]
    fn test_case_1() {
        let result = super::part1("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.\nBlueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.");
        assert_eq!(result, Ok(33));
    }

    #[test]
    fn test_case_2() {
        let result = super::part2("Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.\nBlueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.");
        assert_eq!(result, Ok(56 * 62));
    }
}
