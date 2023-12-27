use std::{
    collections::HashMap,
    io::{self, BufRead},
};

use num::integer;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

type Node = String;

#[derive(Debug)]
struct Simulator {
    routes: HashMap<Node, (Node, Node)>,
}

impl Simulator {
    fn parse(lines: impl Iterator<Item = String>) -> Self {
        Self {
            routes: lines
                .map(|line| Self::parse_line(&line))
                .collect::<HashMap<_, _>>(),
        }
    }

    fn parse_line(line: &str) -> (Node, (Node, Node)) {
        let (from, to) = line
            .split_once(" = ")
            .map(|(from, to)| {
                let (left, right) = to
                    .trim_matches(|c| c == '(' || c == ')')
                    .split_once(", ")
                    .unwrap();
                (from, (left, right))
            })
            .unwrap();

        (String::from(from), (String::from(to.0), String::from(to.1)))
    }

    fn simulate(&self, navigation: &str) -> u64 {
        let starts = self
            .routes
            .iter()
            .map(|(from, _)| from)
            .filter(|from| from.ends_with('A'))
            .collect::<Vec<_>>();

        starts
            .par_iter()
            .map(|start| self.simulate_one(&start, navigation))
            .reduce_with(|a, b| integer::lcm(a, b))
            .unwrap()
    }

    fn simulate_one(&self, start: &str, navigation: &str) -> u64 {
        let mut step = 0;
        let mut state = start;

        for instruction in navigation.chars().cycle() {
            match instruction {
                'L' => state = &self.routes[state].0,
                'R' => state = &self.routes[state].1,
                _ => unreachable!("unexpected instruction {}", instruction),
            }
            step += 1;

            if state.ends_with('Z') {
                break;
            }
        }

        step
    }
}

fn main() {
    let mut lines = io::stdin().lock().lines().map(Result::unwrap);
    let navigation = lines.next().unwrap();
    lines.next();
    let simulator = Simulator::parse(lines);
    println!("{}", simulator.simulate(&navigation));
}
