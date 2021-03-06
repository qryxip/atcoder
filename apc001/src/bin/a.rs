use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
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
      x: usize,
      y: usize,
    }
    if x % y == 0 {
        println!("-1");
        return;
    }
    let mut k = 1;
    while x * k % y == 0 {
        k += 1;
    }
    println!("{}", x * k);
}
