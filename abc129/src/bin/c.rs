use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }
    let a = a.into_iter().collect::<HashSet<_>>();
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    for i in 0..n {
        if !a.contains(&(i + 1)) {
            dp[i + 1] = (dp[i + 1] + dp[i]) % M;
        }
        if i + 2 <= n && !a.contains(&(i + 2)) {
            dp[i + 2] = (dp[i + 2] + dp[i]) % M;
        }
    }
    println!("{}", dp[n]);
}
