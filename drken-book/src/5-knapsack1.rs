use proconio::input;

fn main() {
    input! {
        n: usize,
        weight: u64,
        wv: [(u64, u64); n],
    }

    let mut dp = vec![vec![0; weight as usize + 1]; n + 1];
    for w in 0..weight as usize {
        dp[0][w] = 0;
    }

    for (i, &(w, v)) in wv.iter().enumerate() {
        for j in 0..=weight as usize {
            if j >= w as usize {
                dp[i + 1][j] = (dp[i][j - w as usize] + v).max(dp[i][j]);
            } else {
                dp[i + 1][j] = dp[i][j];
            }
        }
    }

    println!("{}", dp[n][weight as usize]);
}
