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
    }
    if k == 0 {
        println!("{}", n * n);
        return;
    }
    let mut count = 0;
    for b in k + 1..=n {
        count += (b - k) * (n / b);
        if n % b + 1 > k {
            count += n % b + 1 - k;
        }
    }
    println!("{}", count);
}
