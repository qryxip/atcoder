#[allow(unused_imports)]
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::f64::consts::*;
use std::io::*;

#[allow(unused)]
const INF: usize = std::usize::MAX / 4;
#[allow(unused)]
const M: usize = 1000000007;

fn read() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().to_string()
}

fn main() {
    let n = read().parse::<usize>().unwrap();
    let vacant = "Vacant";
    println!("0");
    stdout().flush().ok();
    let s0 = read();
    if s0 == vacant {
        return;
    }
    println!("{}", n - 1);
    stdout().flush().ok();
    let sn = read();
    if sn == vacant {
        return;
    }
    let mut l = 0;
    let mut r = n - 1;
    while l + 1 < r {
        let m = (l + r) / 2;
        println!("{}", m);
        stdout().flush().ok();
        let sm = read();
        if sm == vacant {
            return;
        }
        if (m % 2 == 0 && sm == s0) || (m % 2 == 1 && sm == sn) {
            l = m;
        } else {
            r = m;
        }
    }
}
