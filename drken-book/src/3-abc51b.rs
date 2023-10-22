// 章末3.6
use proconio::input;

fn main() {
    input! {
        k: i32,
        s: i32,
    }

    let mut count = 0;

    for x in 0..=k {
        for y in 0..=k {
            if 0 <= s - x - y && s - x - y <= k {
                count += 1;
            }
        }
    }

    println!("{count}");
}
