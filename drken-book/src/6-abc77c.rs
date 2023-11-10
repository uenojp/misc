use std::cmp::Ordering;

use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n:usize,
        mut a:[u64; n],
        b:[u64; n],
        mut c:[u64; n],
    }
    a.sort();
    c.sort();

    let mut count = 0;

    if false {
        for bi in b {
            let lower = a
                .binary_search_by(|&ai| {
                    if ai < bi {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                })
                .unwrap_or_else(|i| i); // #{i|a[i]<b[i]}
            let upper = c
                .binary_search_by(|&ci| {
                    if ci <= bi {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                })
                .unwrap_or_else(|i| i); // #{i|c[i]<=b[i]}
            let upper = n - upper; // #{i|!(c[i]<=b[i])} == #{i|c[i]>b[i]}

            eprintln!("ci: {} lower: {:?}", bi, lower);
            eprintln!("ci: {} upper: {:?}", bi, upper);

            count += lower * upper;
        }
    } else {
        for bi in b {
            let lower = a.lower_bound(&bi);
            let upper = n - c.upper_bound(&bi);

            eprintln!("ci: {} lower: {:?}", bi, lower);
            eprintln!("ci: {} upper: {:?}", bi, upper);

            count += lower * upper;
        }
    }

    println!("{:?}", count);
}
