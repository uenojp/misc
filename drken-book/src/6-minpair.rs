use proconio::input;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        (n, k): (usize, i64),
        a: [i64; n],
        mut b: [i64; n],
    }
    b.sort();

    let mut min = i64::MAX;
    for ai in a {
        let bi = k - ai;
        match b.binary_search(&bi) {
            Ok(_) => {
                min = k;
                break;
            }
            Err(i) => {
                if i >= b.len() {
                    continue;
                }
                min = min.min(ai + b[i]);
            }
        }
    }

    println!("{:?}", min);

    Ok(())
}
