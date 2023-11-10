use proconio::input;

fn is_ok(h: &[u64], s: &[u64], height_limit: u64) -> bool {
    if h.iter().any(|&hi| hi > height_limit) {
        return false;
    }

    let mut deadlines = h
        .iter()
        .zip(s)
        .map(|(hi, si)| (height_limit - hi) / si)
        .collect::<Vec<_>>();

    deadlines.sort();

    deadlines
        .iter()
        .enumerate()
        .all(|(clock, &deadline)| clock as u64 <= deadline)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        n: usize,
        hs: [(u64, u64); n]
    }
    let (h, s): (Vec<_>, Vec<_>) = hs.into_iter().unzip();

    let mut ng = 0;
    let mut ok = h
        .iter()
        .zip(&s)
        .map(|(hi, si)| hi + si * n as u64)
        .max()
        .unwrap();

    while ok.abs_diff(ng) > 1 {
        let mid = (ok - ng) / 2 + ng;
        if is_ok(&h, &s, mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{:?}", ok);

    Ok(())
}
