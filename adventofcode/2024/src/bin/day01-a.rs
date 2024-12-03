use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = io::stdin().lock().lines().map(Result::unwrap);

    let (mut left, mut right) = (Vec::new(), Vec::new());
    for line in lines {
        let (a, b) = line.split_once("   ").expect("split_once: failed");
        left.push(a.parse::<i32>()?);
        right.push(b.parse::<i32>()?);
    }

    left.sort();
    right.sort();

    let answer = left
        .into_iter()
        .zip(right.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum::<u32>();

    println!("{answer}");

    Ok(())
}
