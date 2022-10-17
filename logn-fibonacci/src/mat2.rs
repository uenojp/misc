use num::{bigint::ToBigUint, BigUint};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Mat2([BigUint; 4]);

impl Mat2 {
    pub fn new(e11: BigUint, e12: BigUint, e21: BigUint, e22: BigUint) -> Mat2 {
        Mat2([e11, e12, e21, e22])
    }

    pub fn e11(&self) -> &BigUint {
        &self.0[0]
    }

    pub fn identity() -> Mat2 {
        Mat2([
            1.to_biguint().unwrap(),
            0.to_biguint().unwrap(),
            0.to_biguint().unwrap(),
            1.to_biguint().unwrap(),
        ])
    }

    pub fn pow(&self, n: u128) -> Mat2 {
        if n == 0 {
            return Self::identity();
        }
        let mut result = Self::identity();
        let mut x = self.clone();
        let mut n = n;
        while n > 0 {
            if n % 2 == 1 {
                result = &result * &x;
            }
            x = &x * &x;
            n /= 2
        }
        result
    }
}

impl std::ops::Mul<&Mat2> for &Mat2 {
    type Output = Mat2;
    fn mul(self, rhs: &Mat2) -> Self::Output {
        Mat2([
            &self.0[0] * &rhs.0[0] + &self.0[1] * &rhs.0[2],
            &self.0[0] * &rhs.0[1] + &self.0[1] * &rhs.0[3],
            &self.0[2] * &rhs.0[0] + &self.0[3] * &rhs.0[2],
            &self.0[2] * &rhs.0[1] + &self.0[3] * &rhs.0[3],
        ])
    }
}

#[cfg(test)]
mod test {
    use num::bigint::ToBigUint;

    use super::Mat2;

    #[test]
    fn test_mul() {
        let a = Mat2::new(
            1.to_biguint().unwrap(),
            9.to_biguint().unwrap(),
            2.to_biguint().unwrap(),
            5.to_biguint().unwrap(),
        );
        let b = Mat2::new(
            9.to_biguint().unwrap(),
            5.to_biguint().unwrap(),
            1.to_biguint().unwrap(),
            3.to_biguint().unwrap(),
        );
        let expected = Mat2::new(
            18.to_biguint().unwrap(),
            32.to_biguint().unwrap(),
            23.to_biguint().unwrap(),
            25.to_biguint().unwrap(),
        );
        assert_eq!(expected, &a * &b);

        let a = Mat2::new(
            0.to_biguint().unwrap(),
            0.to_biguint().unwrap(),
            0.to_biguint().unwrap(),
            0.to_biguint().unwrap(),
        );
        let b = Mat2::new(
            0.to_biguint().unwrap(),
            0.to_biguint().unwrap(),
            0.to_biguint().unwrap(),
            0.to_biguint().unwrap(),
        );
        let expected = Mat2::new(
            0.to_biguint().unwrap(),
            0.to_biguint().unwrap(),
            0.to_biguint().unwrap(),
            0.to_biguint().unwrap(),
        );
        assert_eq!(expected, &a * &b);

        let a = Mat2::new(
            1.to_biguint().unwrap(),
            0.to_biguint().unwrap(),
            0.to_biguint().unwrap(),
            1.to_biguint().unwrap(),
        );
        let b = Mat2::new(
            2.to_biguint().unwrap(),
            4.to_biguint().unwrap(),
            8.to_biguint().unwrap(),
            5.to_biguint().unwrap(),
        );
        let expected = b.clone();
        assert_eq!(expected, &a * &b);

        let a = Mat2::new(
            2.to_biguint().unwrap(),
            4.to_biguint().unwrap(),
            8.to_biguint().unwrap(),
            5.to_biguint().unwrap(),
        );
        let b = Mat2::new(
            1.to_biguint().unwrap(),
            0.to_biguint().unwrap(),
            0.to_biguint().unwrap(),
            1.to_biguint().unwrap(),
        );
        let expected = a.clone();
        assert_eq!(expected, &a * &b);
    }

    #[test]
    fn test_pow() {
        let a = Mat2::new(
            2.to_biguint().unwrap(),
            0.to_biguint().unwrap(),
            0.to_biguint().unwrap(),
            3.to_biguint().unwrap(),
        );
        let n = 10;
        let expected = Mat2::new(
            1024.to_biguint().unwrap(),
            0.to_biguint().unwrap(),
            0.to_biguint().unwrap(),
            59049.to_biguint().unwrap(),
        );
        assert_eq!(expected, a.pow(n));

        let a = Mat2::identity();
        let n = 10;
        let expected = Mat2::identity();
        assert_eq!(expected, a.pow(n));

        let a = Mat2::new(
            2.to_biguint().unwrap(),
            3.to_biguint().unwrap(),
            4.to_biguint().unwrap(),
            5.to_biguint().unwrap(),
        );
        let n = 16;
        let expected = Mat2::new(
            18547144791676u128.to_biguint().unwrap(),
            24458663349897u128.to_biguint().unwrap(),
            32611551133196u128.to_biguint().unwrap(),
            43005808141573u128.to_biguint().unwrap(),
        );

        assert_eq!(expected, a.pow(n));

        let a = Mat2::new(
            2.to_biguint().unwrap(),
            3.to_biguint().unwrap(),
            4.to_biguint().unwrap(),
            5.to_biguint().unwrap(),
        );
        let n = 10;
        let expected = Mat2::new(
            125114812.to_biguint().unwrap(),
            164992569.to_biguint().unwrap(),
            219990092.to_biguint().unwrap(),
            290107381.to_biguint().unwrap(),
        );
        assert_eq!(expected, a.pow(n));
    }
}
