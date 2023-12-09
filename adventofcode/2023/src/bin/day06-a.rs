use std::io::{self, BufRead};

fn main() {
    let mut lines = io::stdin().lock().lines().map(Result::unwrap);

    let times = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let distances = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut winning_ways = vec![0u64; times.len()];
    for (i, (&time, &distance)) in times.iter().zip(distances.iter()).enumerate() {
        for holding_time in 0..=time {
            let velocity = holding_time;
            if velocity * (time - holding_time) > distance {
                winning_ways[i] += 1;
            }
        }
    }

    // eprintln!("{:?} {:?}", times, distances);
    // eprintln!("{:?}", winning_ways);
    println!("{}", winning_ways.iter().product::<u64>());
}
