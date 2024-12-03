use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = io::stdin().lock().lines().map(Result::unwrap);

    let (mut left, mut right) = (Vec::new(), HashMap::new());
    for line in lines {
        let (a, b) = line.split_once("   ").expect("split_once: failed");
        left.push(a.parse::<i32>()?);
        right
            .entry(b.parse::<i32>()?)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let answer = left
        .into_iter()
        .map(|a| a * right.get(&a).unwrap_or(&0))
        .sum::<i32>();

    println!("{answer}");

    Ok(())
}
