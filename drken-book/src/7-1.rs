use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
        mut b: [i64; n],
    }
    a.sort();
    b.sort();

    let mut i = 0;
    for bi in b {
        if a[i] < bi {
            i += 1;
        }
    }
    print!("{}", i);
}
