use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        h: [i64; n],
    }

    let mut dp = vec![i64::MAX / 2; n];
    dp[0] = 0;

    for i in 1..n {
        println!("{:?}", &dp);
        for j in 1..=k as usize {
            // dp[i-1] = 足場iまでで支払うコストの最小値
            // dp[i-1] = min(dp[i-j] + |h[i] - h[j]|) for 1<=j<=k
            if i >= j {
                dp[i] = dp[i].min(dp[i - j] + h[i].abs_diff(h[i - j]) as i64);
            }
        }
    }
    println!("{:?}", dp);

    println!("{}", dp[n - 1]);
}
