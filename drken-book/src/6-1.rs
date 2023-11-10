use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n:usize,
        a:[i64; n]
    }

    let mut sorted = a.clone();
    sorted.sort();
    let sorted = sorted.into_iter().unique().collect::<Vec<_>>();

    let indeces = a
        .iter()
        .map(|ai| sorted.binary_search(ai).unwrap())
        .collect::<Vec<_>>();

    for i in indeces {
        println!("{}", i);
    }
}
