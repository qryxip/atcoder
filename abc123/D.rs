#[allow(unused_macros)]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };

    ($next:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, [ $t:tt ]) => {
        {
            let len = read_value!($next, usize);
            (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
        }
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, bytes) => {
        read_value!($next, String).into_bytes()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn check(a: &Vec<usize>, b: &Vec<usize>, c: &Vec<usize>, p: usize, k: usize) -> bool {
    let mut count = 0;
    for &ai in a {
        for &bj in b {
            for &ck in c {
                if ai + bj + ck < p {
                    break;
                }
                count += 1;
                if count >= k {
                    return true;
                }
            }
        }
    }
    false
}

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
        k: usize,
        a: [usize; x],
        b: [usize; y],
        c: [usize; z],
    }
    let mut a = a;
    let mut b = b;
    let mut c = c;
    a.sort();
    a.reverse();
    b.sort();
    b.reverse();
    c.sort();
    c.reverse();

    let max = a[0] + b[0] + c[0];
    let mut left = 0;
    let mut right = max;
    while left != right {
        let p = (left + right) / 2;
        if check(&a, &b, &c, p, k) {
            left = p + 1;
        } else {
            right = p;
        }
    }
    let p = left - 1;
    // println!("{}", p);

    let mut items = Vec::new();
    for &ai in &a {
        for &bj in &b {
            for &ck in &c {
                if ai + bj + ck < p {
                    break;
                }
                items.push(ai + bj + ck);
            }
        }
    }
    items.sort();
    items.reverse();
    for i in 0..k {
        println!("{}", items[i]);
    }
}
