// 章末3.7
use proconio::input;

fn main() {
    input! {
        s : String,
    }

    let len = s.len();
    let digits = s
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect::<Vec<_>>();
    let mut sum = 0;

    for bit in 0..(1 << (len - 1)) {
        let mut term = 0;
        for i in 0..(len - 1) {
            term *= 10;
            term += digits[i];

            if (bit & (1 << i)) != 0 {
                sum += term;
                term = 0;
            }
        }

        term *= 10;
        term += digits[len - 1];
        sum += term;
    }

    println!("{sum}");
}
