use num::bigint::ToBigInt;
use num::BigInt;
use num::{One, Zero};

#[derive(Debug)]
struct PiCalculator {
    digit: u32,
    formula: Vec<(BigInt, BigInt)>,
}

impl PiCalculator {
    const fn new(digit: u32, formula: Vec<(BigInt, BigInt)>) -> Self {
        Self { digit: digit + 5, formula }
    }

    fn calculate(&self) -> BigInt {
        let mut pi: BigInt = Zero::zero();
        let mut sign: BigInt = One::one();
        let base = (10.to_bigint().unwrap()).pow(self.digit);

        let mut n = 1;
        let mut a_pows: Vec<BigInt> = self.formula.iter().map(|(_, a)| a.clone()).collect();
        let mut term_results: Vec<BigInt> = vec![Zero::zero(); self.formula.len()];
        loop {
            for (id, (k, a)) in self.formula.iter().enumerate() {
                term_results[id] = &base * &sign * k / &a_pows[id] / n;
                pi += &term_results[id];
                a_pows[id] *= a * a;
            }

            if term_results.iter().all(|result| result.is_zero()) {
                break;
            }

            sign *= -1;
            n += 2;
        }
        pi
    }
}

fn main() {
    println!(
        "{}",
        PiCalculator::new(
            10000,
            // pi/4 = 44*atan(1/49) + 7*atan(1/57) - 12*atan(1/239) * 12*atan(1/12943)
            // pi = 4 * (44*atan(1/49) + 7*atan(1/57) - 12*atan(1/239) * 12*atan(1/12943))
            // ref. https://xn--w6q13e505b.jp/formula/arctan.html
            vec![(44, 57), (7, 239), (-12, 682), (24, 12943)]
                .into_iter()
                .map(|(a, b)| (4 * a.to_bigint().unwrap(), b.to_bigint().unwrap()))
                .collect()
        )
        .calculate()
    );
}
