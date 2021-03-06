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
    }
    let mut f = vec![0usize; n + 1];
    for i in 1..=n {
        for j in (i..=n).step_by(i) {
            f[j] += 1;
        }
    }
    let mut result = 0;
    for i in 1..=n {
        result += i * f[i];
    }
    println!("{}", result);
}
