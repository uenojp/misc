use proconio::input;

fn dfs(h: &[u64], i: usize) -> u64 {
    if i == 0 {
        return 0;
    }

    let cost1 = dfs(h, i - 1) + h[i - 1].abs_diff(h[i]);
    if i > 1 {
        let cost2 = dfs(h, i - 2) + h[i - 2].abs_diff(h[i]);
        cost1.min(cost2)
    } else {
        cost1
    }
}

fn memo_dfs(h: &[u64], i: usize, dp: &mut [Option<u64>]) -> u64 {
    if i == 0 {
        return 0;
    }

    if let Some(v) = dp[i] {
        return v;
    }

    let cost1 = memo_dfs(h, i - 1, dp) + h[i - 1].abs_diff(h[i]);
    if i > 1 {
        let cost2 = memo_dfs(h, i - 2, dp) + h[i - 2].abs_diff(h[i]);
        dp[i] = Some(cost1.min(cost2));
    } else {
        dp[i] = Some(cost1);
    }
    dp[i].unwrap()
}

fn main() {
    input! {
        n: usize,
        h: [u64; n],
    }

    let mut dp = vec![u64::MAX; n];
    dp[0] = 0;
    for i in 0..n - 1 {
        let cost1 = dp[i] + h[i].abs_diff(h[i + 1]);
        dp[i + 1] = cost1.min(dp[i + 1]);

        if i + 2 < n {
            let cost2 = dp[i] + h[i].abs_diff(h[i + 2]);
            dp[i + 2] = cost2.min(dp[i + 2]);
        }
    }
    println!("{}", dp[n - 1]);

    let mut dp = vec![u64::MAX; n];
    dp[0] = 0;
    for i in 1..n {
        let cost1 = dp[i - 1] + h[i - 1].abs_diff(h[i]);
        if i > 1 {
            let cost2 = dp[i - 2] + h[i - 2].abs_diff(h[i]);
            dp[i] = cost1.min(cost2);
        } else {
            dp[i] = cost1;
        }
    }
    println!("{}", dp[n - 1]);

    println!("{}", dfs(&h, n - 1));

    let mut dp = vec![None; n];
    println!("{}", memo_dfs(&h, n - 1, &mut dp));
}
