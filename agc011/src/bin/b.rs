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
        mut a: [usize; n],
    }
    a.sort();
    let mut acc = vec![0usize; n];
    acc[0] = a[0];
    for i in 1..n {
        acc[i] = acc[i - 1] + a[i];
    }
    for i in (0..n - 1).rev() {
        if acc[i] * 2 < a[i + 1] {
            println!("{}", n - i - 1);
            return;
        }
    }
    println!("{}", n);
}
