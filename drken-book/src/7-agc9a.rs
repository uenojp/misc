use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(u64, u64); n],
    }

    let mut count = 0;
    for &(a, b) in ab.iter().rev() {
        eprintln!("rem {}", (a + b - 1) / b * b - a);
        let a = a + count;
        count += (a + b - 1) / b * b - a;
    }

    println!("{:?}", count);
}
