use num::{bigint::ToBigUint, BigUint};

use crate::mat2::Mat2;

// F_0 = 1, F_1 = 1, F_n+2 = F_n+1 + F_n
// (F_n+2 F_n+1)   (1 1)   (F_n+3 F_n+2)
// (F_n+1 F_n  ) * (1 0) = (F_n+2 F_n+1)
pub fn fibonacci(n: u128) -> BigUint {
    Mat2::new(
        1.to_biguint().unwrap(),
        1.to_biguint().unwrap(),
        1.to_biguint().unwrap(),
        0.to_biguint().unwrap(),
    )
    .pow(n)
    .e11()
    .clone()
}
