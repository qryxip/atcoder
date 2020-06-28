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

fn inv(a: usize) -> usize {
    let m = M as i64;
    let mut a = a as i64;
    let mut b = m as i64;
    let mut u = 1;
    let mut v = 0;
    let mut tmp;
    while b != 0 {
        let t = a / b;
        a -= t * b;
        tmp = a;
        a = b;
        b = tmp;
        u -= t * v;
        tmp = u;
        u = v;
        v = tmp;
    }
    u %= m;
    if u < 0 {
        u += m;
    }
    return u as usize;
}

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut f = vec![1; m + 1];
    let mut g = vec![1; m + 1];
    for i in 1..=m {
        f[i] = i * f[i - 1] % M;
        g[i] = inv(f[i]);
    }
    let mut result = 0;
    for k in 0..=n {
        let s = f[m - k] * g[m - n] % M;
        let s = s * s % M;
        let t = f[n] * g[k] % M * g[n - k] % M;
        let u = f[m] * g[m - k] % M;
        // eprintln!("{} {} {}", s, t, u);
        if k % 2 == 0 {
            result = (result + s * t % M * u % M) % M;
        } else {
            result = (result + M - s * t % M * u % M) % M;
        }
    }
    println!("{}", result);
}
