use proconio::input;

fn main() {
    input! {
        n:usize,
        mut a:[u64; n],
        mut b:[u64; n],
        mut c:[u64; n],
    }
    a.sort();
    b.sort();
    c.sort();
}
