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
        mut s: Chars,
        mut t: Chars,
    }
    s.sort();
    t.sort();
    t.reverse();
    let s = s.into_iter().collect::<String>();
    let t = t.into_iter().collect::<String>();
    if s < t {
        println!("Yes");
    } else {
        println!("No");
    }
}
