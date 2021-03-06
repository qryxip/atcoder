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
        k: usize,
        x: [usize; n],
    }
    let mut s = 0;
    for i in 0..n {
        if x[i] < k - x[i] {
            s += x[i];
        } else {
            s += k - x[i];
        }
    }
    println!("{}", 2 * s);
}
