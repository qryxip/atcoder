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
        t: usize,
        a: [usize; n],
    }
    let mut s = 0;
    for i in 1..n {
        if a[i] - a[i - 1] >= t {
            s += t;
        } else {
            s += a[i] - a[i - 1];
        }
    }
    s += t;
    println!("{}", s);
}
