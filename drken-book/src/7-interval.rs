use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ab: [(u64, u64); n],
    }
    ab.sort_by(|&x, &y| x.1.cmp(&y.1));

    let mut count = 0;
    let mut end = 0;
    for interval in ab {
        if end <= interval.0 {
            count += 1;
            end = interval.1;
        }
    }

    println!("{:?}", count);
}
