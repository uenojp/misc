use std::io::{self, BufRead};

fn is_safe(levels: &[i32]) -> bool {
    let inc = levels
        .windows(2)
        .all(|ab| (ab[0] < ab[1]) && (ab[0].abs_diff(ab[1]) <= 3));
    let dec = levels
        .windows(2)
        .all(|ab| (ab[0] > ab[1]) && (ab[0].abs_diff(ab[1]) <= 3));

    inc || dec
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = io::stdin().lock().lines().map(Result::unwrap);

    let mut answer = 0;
    for line in lines {
        let levels = line
            .split_whitespace()
            .map(|n| n.parse::<i32>())
            .collect::<Result<Vec<i32>, _>>()?;

        if is_safe(&levels) {
            answer += 1;
        } else {
            for i in 0..levels.len() {
                let l = levels
                    .iter()
                    .enumerate()
                    .filter(|(j, _)| i != *j)
                    .map(|(_, level)| *level)
                    .collect::<Vec<_>>();
                if is_safe(&l) {
                    answer += 1;
                    break;
                }
            }
        }
    }

    println!("{answer}");

    Ok(())
}
