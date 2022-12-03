#![feature(iter_array_chunks)]

use std::collections::{HashMap, HashSet};

fn main() {
    let priority =
        HashMap::<char, i32>::from_iter(('a'..='z').chain('A'..='Z').zip((1..=26).chain(27..=52)));

    let sum = include_str!("../input")
        .lines()
        .array_chunks::<3>()
        .map(|[first, second, third]| {
            let first = HashSet::<char>::from_iter(first.chars());
            let second = HashSet::<char>::from_iter(second.chars());
            let third = HashSet::<char>::from_iter(third.chars());

            let i1 = first.intersection(&second).collect::<HashSet<_>>();
            let i2 = first.intersection(&third).collect::<HashSet<_>>();

            **i1.intersection(&i2).next().unwrap()
        })
        .map(|c| priority.get(&c).unwrap())
        .sum::<i32>();

    println!("{:?}", sum);
}
