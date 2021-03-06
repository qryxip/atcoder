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
        p: [usize; n],
    }
    let mut count = 0;
    for i in 2..n {
        if (p[i - 2] < p[i - 1] && p[i - 1] < p[i]) || (p[i - 2] > p[i - 1] && p[i - 1] > p[i]) {
            count += 1;
        }
    }
    println!("{}", count);
}
