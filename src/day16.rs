use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    num::ParseIntError,
    str::FromStr,
};

use aoc_runner_derive::aoc;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct ValveId(u16);

impl Debug for ValveId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            String::from_utf8(self.0.to_be_bytes().into()).unwrap()
        )
    }
}

impl FromStr for ValveId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.as_bytes();

        Ok(ValveId(u16::from_be_bytes([s[0], s[1]])))
    }
}

#[derive(Debug)]
struct Valve {
    id: ValveId,
    flow_rate: u8,
    tunnels: Vec<ValveId>,
}

impl FromStr for Valve {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(s) = s.strip_prefix("Valve ") else {
            panic!("Invalid input");
        };

        let (id, s) = s.split_at(2);

        let Some(s) = s.strip_prefix(" has flow rate=") else {
            panic!("Invalid input");
        };

        let Some((flow_rate, s)) = s.split_once(';') else {
            panic!("Invalid input");
        };

        let Some(s) = s.strip_prefix(" tunnel") else {
            panic!("Invalid input");
        };

        let s = s.trim_start_matches('s');

        let Some(s) = s.strip_prefix(" lead") else {
            panic!("Invalid input");
        };

        let s = s.trim_start_matches('s');

        let Some(s) = s.strip_prefix(" to valve") else {
            panic!("Invalid input");
        };

        let s = s.trim_start_matches('s');

        let tunnels = s
            .split(',')
            .map(|s| s.trim().parse::<ValveId>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Valve {
            id: id.parse()?,
            flow_rate: flow_rate.parse()?,
            tunnels,
        })
    }
}

struct Step {
    open: HashSet<u8>,
    position: ValveId,
    previous: ValveId,
    score: u64,
    time_left: u8,
}

impl Step {
    fn new() -> Self {
        Self {
            open: HashSet::new(),
            position: "AA".parse().unwrap(),
            previous: "AA".parse().unwrap(),
            score: 0,
            time_left: 30,
        }
    }

    fn open(&self, valve: &Valve) -> Self {
        let time_left = self.time_left - 1;
        let mut open = self.open.clone();

        open.insert(valve.flow_rate);

        Self {
            open,
            position: self.position,
            previous: self.position,
            score: self.score + ((valve.flow_rate as u64) * (time_left as u64)),
            time_left,
        }
    }

    fn goto(&self, position: ValveId) -> Self {
        Self {
            open: self.open.clone(),
            position,
            previous: self.position,
            score: self.score,
            time_left: self.time_left - 1,
        }
    }

    fn is_closed(&self, valve: &Valve) -> bool {
        !self.open.contains(&valve.flow_rate)
    }

    fn best(&self, sorted_flow_rates: &[u8]) -> u64 {
        let mut time_left = self.time_left as u64;
        let mut score = self.score;

        for &flow_rate in sorted_flow_rates {
            if self.open.contains(&flow_rate) {
                continue;
            }

            time_left -= 1;
            if time_left == 0 {
                break;
            }

            score += (flow_rate as u64) * time_left;

            time_left -= 1;
            if time_left == 0 {
                break;
            }
        }

        score
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> Result<u64, ParseIntError> {
    let valves = input
        .lines()
        .map(Valve::from_str)
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|valve| (valve.id, valve))
        .collect::<HashMap<_, _>>();

    let max_open = valves.values().filter(|valve| valve.flow_rate > 0).count();

    let mut sorted_flow_rates = valves
        .values()
        .map(|valve| valve.flow_rate)
        .filter(|&flow_rate| flow_rate > 0)
        .collect::<Vec<_>>();

    sorted_flow_rates.sort();
    sorted_flow_rates.reverse();

    let mut result = 0;
    let mut queue = vec![Step::new()];

    while let Some(step) = queue.pop() {
        if step.time_left == 0 {
            result = result.max(step.score);
            continue;
        }

        if step.open.len() == max_open {
            result = result.max(step.score);
            continue;
        }

        if step.best(&sorted_flow_rates) < result {
            continue;
        }

        let valve = &valves[&step.position];

        if valve.flow_rate > 0 && step.is_closed(valve) {
            queue.push(step.open(valve));
        }

        for tunnel in valve.tunnels.iter() {
            if *tunnel != step.previous {
                queue.push(step.goto(*tunnel));
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        let result = super::part1("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\nValve BB has flow rate=13; tunnels lead to valves CC, AA\nValve CC has flow rate=2; tunnels lead to valves DD, BB\nValve DD has flow rate=20; tunnels lead to valves CC, AA, EE\nValve EE has flow rate=3; tunnels lead to valves FF, DD\nValve FF has flow rate=0; tunnels lead to valves EE, GG\nValve GG has flow rate=0; tunnels lead to valves FF, HH\nValve HH has flow rate=22; tunnel leads to valve GG\nValve II has flow rate=0; tunnels lead to valves AA, JJ\nValve JJ has flow rate=21; tunnel leads to valve II");
        assert_eq!(result, Ok(1651));
    }
}
