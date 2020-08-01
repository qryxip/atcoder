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
        a: [isize; n],
    }
    let mut x = 0;
    for i in 0..n {
        if i % 2 == 0 {
            x += a[i];
        } else {
            x -= a[i];
        }
    }
    println!("{}", x);
    for i in 1..n {
        x = 2 * a[i - 1] - x;
        println!("{}", x);
    }
}
