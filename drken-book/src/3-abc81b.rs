// 章末3.5
use proconio::input;

fn main() {
    input! {
        n : usize,
        mut a : [u64; n],
    }

    let mut count = 0;
    while a.iter().all(|ai| ai % 2 == 0) {
        a = a.into_iter().map(|ai| ai / 2).collect::<Vec<_>>();
        count += 1;
    }

    println!("{count}");
}
