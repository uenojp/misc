use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = io::stdin().lock().lines().map(Result::unwrap);

    let mut answer = 0;
    for line in lines {
        let levels = line
            .split_whitespace()
            .map(|n| n.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()?;

        let inc = levels
            .windows(2)
            .all(|ab| (ab[0] < ab[1]) && (ab[0].abs_diff(ab[1]) <= 3));
        let dec = levels
            .windows(2)
            .all(|ab| (ab[0] > ab[1]) && (ab[0].abs_diff(ab[1]) <= 3));
        answer += (inc || dec) as i32;
    }

    println!("{answer}");

    Ok(())
}
