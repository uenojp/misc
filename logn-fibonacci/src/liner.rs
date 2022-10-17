use num::{bigint::ToBigUint, BigUint};

pub fn fibonacci(n: u128) -> BigUint {
    let mut f0 = 1.to_biguint().unwrap();
    let mut f1 = 1.to_biguint().unwrap();
    for _ in 0..n {
        let tmp = f0;
        f0 = f1.clone();
        f1 = &tmp + &f1;
    }
    f0
}
