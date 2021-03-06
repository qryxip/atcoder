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

fn prime_factors(n: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let mut m = n;
    for d in 2.. {
        if d * d > n {
            break;
        }
        let mut count = 0;
        while m % d == 0 {
            count += 1;
            m /= d;
        }
        if count > 0 {
            result.push((d, count));
        }
    }
    if m > 1 {
        result.push((m, 1));
    }
    result
}

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

struct Combination {
    m: usize,
    f: Vec<usize>,
    g: Vec<usize>,
}

impl Combination {
    fn new(m: usize) -> Combination {
        Combination {
            m,
            f: vec![1],
            g: vec![1],
        }
    }

    fn combinations(&mut self, n: usize, k: usize) -> usize {
        for i in self.f.len()..=n {
            self.f.push(self.f[i - 1] * i % self.m);
            self.g.push(inv(self.f[i]));
        }
        self.f[n] * self.g[k] % self.m * self.g[n - k] % self.m
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut c = Combination::new(M);
    let mut result = 1;
    for &(_, k) in &prime_factors(m) {
        result = result * c.combinations(n + k - 1, k) % M
    }
    println!("{}", result);
}
