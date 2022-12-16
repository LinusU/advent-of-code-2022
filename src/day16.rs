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

struct Part1Step {
    open: HashSet<u8>,
    position: ValveId,
    previous: ValveId,
    score: u64,
    time_left: u8,
}

impl Part1Step {
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
    let mut queue = vec![Part1Step::new()];

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

enum Move<'a> {
    Goto(ValveId),
    Open(&'a Valve),
}

struct Part2Step {
    open: HashSet<u8>,
    position: ValveId,
    previous: ValveId,
    elephant_position: ValveId,
    elephant_previous: ValveId,
    score: u64,
    time_left: u8,
}

impl Part2Step {
    fn new() -> Self {
        Self {
            open: HashSet::new(),
            position: "AA".parse().unwrap(),
            previous: "AA".parse().unwrap(),
            elephant_position: "AA".parse().unwrap(),
            elephant_previous: "AA".parse().unwrap(),
            score: 0,
            time_left: 26,
        }
    }

    fn next(&self, my_move: &Move, elephant_move: &Move) -> Self {
        let mut result = Self {
            open: self.open.clone(),
            position: self.position,
            previous: self.position,
            elephant_position: self.elephant_position,
            elephant_previous: self.elephant_position,
            score: self.score,
            time_left: self.time_left - 1,
        };

        match my_move {
            Move::Goto(id) => {
                result.position = *id;
            }
            Move::Open(valve) => {
                assert!(result.open.insert(valve.flow_rate));
                result.position = valve.id;
                result.score += (valve.flow_rate as u64) * (result.time_left as u64);
            }
        }

        match elephant_move {
            Move::Goto(id) => {
                result.elephant_position = *id;
            }
            Move::Open(valve) => {
                assert!(result.open.insert(valve.flow_rate));
                result.elephant_position = valve.id;
                result.score += (valve.flow_rate as u64) * (result.time_left as u64);
            }
        }

        result
    }

    fn is_closed(&self, valve: &Valve) -> bool {
        !self.open.contains(&valve.flow_rate)
    }

    fn best(&self, sorted_flow_rates: &[u8]) -> u64 {
        let mut time_left = self.time_left;
        let mut score = self.score;

        let sorted_flow_rates = sorted_flow_rates
            .iter()
            .filter(|flow_rate| !self.open.contains(flow_rate))
            .collect::<Vec<_>>();

        for flow_rates in sorted_flow_rates.chunks(2) {
            time_left -= 1;
            if time_left == 0 {
                break;
            }

            for &flow_rate in flow_rates {
                score += (*flow_rate as u64) * (time_left as u64);
            }

            time_left -= 1;
            if time_left == 0 {
                break;
            }
        }

        score
    }
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> Result<u64, ParseIntError> {
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
    let mut queue = vec![Part2Step::new()];

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

        if step.position == step.elephant_position {
            let valve = &valves[&step.position];

            if step.is_closed(valve) {
                for &tunnel in valve.tunnels.iter() {
                    if tunnel != step.previous && tunnel != step.elephant_previous {
                        queue.push(step.next(&Move::Open(valve), &Move::Goto(tunnel)));
                    }
                }
            }

            for (idx, &tunnel_a) in valve.tunnels.iter().enumerate() {
                if tunnel_a == step.previous || tunnel_a == step.elephant_previous {
                    continue;
                }

                for &tunnel_b in valve.tunnels.iter().skip(idx) {
                    if tunnel_b == step.previous || tunnel_b == step.elephant_previous {
                        continue;
                    }

                    queue.push(step.next(&Move::Goto(tunnel_a), &Move::Goto(tunnel_b)));
                }
            }
        } else {
            let valve = &valves[&step.position];
            let mut moves = Vec::<Move>::new();

            if valve.flow_rate > 0 && step.is_closed(valve) {
                moves.push(Move::Open(valve));
            }

            for &tunnel in valve.tunnels.iter() {
                if tunnel != step.previous {
                    moves.push(Move::Goto(tunnel));
                }
            }

            let elephant_valve = &valves[&step.elephant_position];
            let mut elephant_moves = Vec::<Move>::new();

            if elephant_valve.flow_rate > 0 && step.is_closed(elephant_valve) {
                elephant_moves.push(Move::Open(elephant_valve));
            }

            for &tunnel in elephant_valve.tunnels.iter() {
                if tunnel != step.elephant_previous {
                    elephant_moves.push(Move::Goto(tunnel));
                }
            }

            for me in moves.iter() {
                for elephant in elephant_moves.iter() {
                    queue.push(step.next(me, elephant))
                }
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

    #[test]
    fn test_case_2() {
        let result = super::part2("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\nValve BB has flow rate=13; tunnels lead to valves CC, AA\nValve CC has flow rate=2; tunnels lead to valves DD, BB\nValve DD has flow rate=20; tunnels lead to valves CC, AA, EE\nValve EE has flow rate=3; tunnels lead to valves FF, DD\nValve FF has flow rate=0; tunnels lead to valves EE, GG\nValve GG has flow rate=0; tunnels lead to valves FF, HH\nValve HH has flow rate=22; tunnel leads to valve GG\nValve II has flow rate=0; tunnels lead to valves AA, JJ\nValve JJ has flow rate=21; tunnel leads to valve II");
        assert_eq!(result, Ok(1707));
    }
}
