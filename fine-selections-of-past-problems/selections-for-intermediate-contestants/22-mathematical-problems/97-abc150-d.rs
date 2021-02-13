use std::{
    cmp::{max, min},
    collections::HashSet,
    io::*,
    str::FromStr,
};

#[allow(unused_macros)]
macro_rules! scan {
  ($e:expr; $t:ty) => {
    $e.get::<$t>()
  };
  ($e:expr; $($t:ty), *) => {
    ($($e.get::<$t>(),)*)
  }
}

struct Scanner<R: BufRead> {
    reader: R,
    iter: std::vec::IntoIter<String>,
}

#[allow(dead_code)]
impl<R: BufRead> Scanner<R> {
    fn new(reader: R) -> Scanner<R> {
        Scanner {
            reader,
            iter: vec![String::new()].into_iter(),
        }
    }
    fn new_line(&mut self) {
        let mut line = String::new();
        self.reader.read_line(&mut line).ok();
        self.iter = line
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .into_iter();
    }
    fn get<T: FromStr>(&mut self) -> T {
        self.iter.next().unwrap().parse().ok().unwrap()
    }
    fn get_as_vec<T: FromStr>(&mut self) -> Vec<T> {
        self.iter.clone().map(|v| v.parse().ok().unwrap()).collect()
    }
    fn get_line(&mut self) -> String {
        let mut line = String::new();
        self.reader.read_line(&mut line).ok();
        line.trim().to_string()
    }
}

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n = sc.get::<u128>();
    let m = sc.get::<u128>();

    sc.new_line();
    let a = sc.get_as_vec::<u128>();

    for &e in &a {
        if e % 2 == 1 {
            println!("0");
            return;
        }
    }

    let mut min_2 = 1_000_000_010;
    let mut max_2 = 0;

    for &e in &a {
        let mut tmp = e;
        let mut num_2 = 0;
        while tmp % 2 == 0 {
            num_2 += 1;
            tmp /= 2;
        }
        min_2 = min(min_2, num_2);
        max_2 = max(max_2, num_2);
    }

    if max_2 != min_2 {
        println!("0");
        return;
    }

    let cut = {
        let mut tmp = 1;
        for &e in &a {
            tmp = lcm(tmp, e / 2);
        }
        tmp
    };
    let mut period = 2 * cut;

    if cut > m {
        println!("0");
        return;
    }

    let mut ans = 1 + (m - cut) / period;

    println!("{}", ans);
}

// gcd
fn gcd(a: u128, b: u128) -> u128 {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}
// lcm
fn lcm(a: u128, b: u128) -> u128 {
    a * b / gcd(a, b)
}
