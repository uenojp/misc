use std::io::{self, BufRead};

fn main() {
    let mut lines = io::stdin().lock().lines().map(Result::unwrap);

    let time = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .replace(" ", "")
        .parse::<u64>()
        .unwrap();

    let mut winning_ways = 0;
    for holding_time in 0..=time {
        let velocity = holding_time;
        if velocity * (time - holding_time) > distance {
            winning_ways += 1;
        }
    }

    // eprintln!("{:?} {:?}", times, distances);
    // eprintln!("{:?}", winning_ways);
    println!("{}", winning_ways);
}
