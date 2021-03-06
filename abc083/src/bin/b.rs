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
        a: usize,
        b: usize,
    }
    let mut total = 0;
    for i in 1..=n {
        let mut x = i;
        let mut s = 0;
        while x > 0 {
            s += x % 10;
            x /= 10;
        }
        if a <= s && s <= b {
            total += i;
        }
    }
    println!("{}", total);
}
