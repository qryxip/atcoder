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
        a: String,
        b: String
    }
    let c = (a + &b).parse::<usize>().ok().unwrap();
    let sqrt_c = (c as f64).sqrt() as usize;
    if c == sqrt_c * sqrt_c {
        println!("Yes");
    } else {
        println!("No");
    }
}
