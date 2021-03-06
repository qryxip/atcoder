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
        mut w: [isize; n],
    }
    let mut result = INF as isize;
    for i in 1..n {
        let s1 = (0..i).map(|j| w[j]).sum::<isize>();
        let s2 = (i..n).map(|j| w[j]).sum::<isize>();
        result = min(result, (s1 - s2).abs());
    }
    println!("{}", result);
}
