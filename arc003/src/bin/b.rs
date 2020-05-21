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
        mut s: [Chars; n],
    }
    for i in 0..n {
        s[i].reverse();
    }
    s.sort();
    for i in 0..n {
        s[i].reverse();
        println!("{}", s[i].iter().collect::<String>());
    }
}
