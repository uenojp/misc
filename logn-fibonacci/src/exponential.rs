use num::{bigint::ToBigUint, BigUint};

pub fn fibonacci(n: u128) -> BigUint {
    if n == 0 || n == 1 {
        return 1.to_biguint().unwrap();
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
