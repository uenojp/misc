use std::io::{self, BufRead};

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);

    let histories = lines
        .map(|line| {
            line.split(' ')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for history in histories.into_iter() {
        let mut difference_histories = Vec::<Vec<i64>>::new();
        let mut difference = history.clone();
        difference_histories.push(difference.iter().rev().cloned().collect());
        while !difference.iter().all(|&n| n == 0) {
            difference = difference
                .windows(2)
                .map(|s| s[1] - s[0])
                .collect::<Vec<_>>();
            difference_histories.push(difference.iter().rev().cloned().collect());
        }

        let len = difference_histories.len();
        difference_histories[len - 1].push(0);
        for rank in (1..len).rev() {
            // NOTE: differences are reversed (see above).
            let upper_left = *difference_histories[rank - 1].last().unwrap();
            let bottom = *difference_histories[rank].last().unwrap();
            difference_histories[rank - 1].push(upper_left - bottom);
        }

        // eprintln!("{:?}", difference_histories);
        sum += difference_histories[0].last().unwrap();
    }

    println!("{:?}", sum);
}
