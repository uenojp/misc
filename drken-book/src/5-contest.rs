use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [u64; n],
    }
    let w = 100 * 100;
    // let w = 20;

    let mut dp = vec![vec![false; w + 1]; n + 1];
    dp[0][0] = true;

    for i in 0..n {
        for j in 0..=w {
            // dp[i][j] = i問目までの問題を何問か解きj点になるかどうか
            if j >= p[i] as usize {
                dp[i + 1][j] |= dp[i][j - p[i] as usize];
            }
            dp[i + 1][j] |= dp[i][j];
        }
    }
    println!("{}", dp[n].iter().filter(|x| **x).count());
}
