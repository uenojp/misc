use proconio::input;

fn main() {
    input! {
        n: usize,
        abc: [(i64, i64, i64); n],
    }

    let mut dp = vec![vec![0; 3]; n + 1];
    dp[0][0] = 0;
    dp[0][1] = 0;
    dp[0][2] = 0;

    for (i, &(ai, bi, ci)) in abc.iter().enumerate() {
        for j in 0..3 {
            match j {
                0 => dp[i + 1][j] = (dp[i][1] + bi).max(dp[i][2] + ci),
                1 => dp[i + 1][j] = (dp[i][0] + ai).max(dp[i][2] + ci),
                2 => dp[i + 1][j] = (dp[i][0] + ai).max(dp[i][1] + bi),
                _ => unreachable!(),
            }
        }
    }

    println!("{}", dp[n].iter().max().unwrap());
}
