use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut dp = vec![vec![usize::MAX / 2; t.len() + 1]; s.len() + 1];
    dp[0][0] = 0;

    for i in 0..=s.len() {
        for j in 0..=t.len() {
            // replace
            if i > 0 && j > 0 {
                if s[i - 1] == t[j - 1] {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j - 1]);
                } else {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j - 1] + 1);
                }
            }

            // insert
            if j > 0 {
                dp[i][j] = dp[i][j].min(dp[i][j - 1] + 1);
            }

            // delete
            if i > 0 {
                dp[i][j] = dp[i][j].min(dp[i - 1][j] + 1);
            }
        }
    }

    println!("{}", dp[s.len()][t.len()]);
}
