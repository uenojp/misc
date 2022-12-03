use std::collections::{HashMap, HashSet};

fn main() {
    let priority =
        HashMap::<char, i32>::from_iter(('a'..='z').chain('A'..='Z').zip((1..=26).chain(27..=52)));

    let sum = include_str!("../input")
        .lines()
        .map(|s| s.split_at(s.len() / 2))
        .map(|(left, right)| {
            let left = HashSet::<char>::from_iter(left.chars());
            let right = HashSet::<char>::from_iter(right.chars());
            *left.intersection(&right).next().unwrap()
        })
        .into_iter()
        .map(|c| priority.get(&c).unwrap())
        .sum::<i32>();

    println!("{:?}", sum);
}
