use std::io::{self, BufRead};

fn main() {
    let lines = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let (width, height) = (lines[0].len(), lines.len());

    for h in 1..height - 1 {
        for w in 1..width - 1 {
        }
    }
}
