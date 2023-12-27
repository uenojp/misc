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
        let mut difference_histories = vec![];
        let mut difference = history.clone();
        difference_histories.push(difference.clone());
        while !difference.iter().all(|&n| n == 0) {
            difference = difference
                .windows(2)
                .map(|s| s[1] - s[0])
                .collect::<Vec<_>>();
            difference_histories.push(difference.clone());
        }

        let len = difference_histories.len();
        difference_histories[len - 1].push(0);
        for rank in (1..len).rev() {
            let upper_right = *difference_histories[rank - 1].last().unwrap();
            let bottom = *difference_histories[rank].last().unwrap();
            difference_histories[rank - 1].push(upper_right + bottom);
        }

        sum += difference_histories[0].last().unwrap();
    }

    println!("{:?}", sum);
}
