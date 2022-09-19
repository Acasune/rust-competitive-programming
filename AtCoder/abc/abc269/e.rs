#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
use std::{
    collections::HashSet,
    io::{self, *},
    str::FromStr,
};
use std::{f64, i64};

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
    let N: i64 = sc.get();

    let mut n_r = N - 1;
    let mut a = 1;
    let mut b = N;
    let mut c = 1;
    let mut d = N;
    while b != a {
        let m = (a + b) / 2;
        let t = interactive(a, m, c, d, &mut sc);
        if m - a + 1 > t {
            b = m;
            n_r = t
        } else {
            a = m + 1;
            n_r = n_r - t;
        }
    }
    let ans1 = a;
    a = 1;
    b = N;
    n_r = N - 1;
    while d != c {
        let m = (c + d) / 2;
        let t = interactive(a, b, c, m, &mut sc);
        if m - c + 1 > t {
            d = m;
            n_r = t
        } else {
            c = m + 1;
            n_r = n_r - t;
        }
    }
    let ans2 = c;
    println!("! {} {}", ans1, ans2);
}

fn interactive(a: i64, b: i64, c: i64, d: i64, sc: &mut Scanner<StdinLock>) -> i64 {
    println!("? {} {} {} {}", a, b, c, d);
    sc.new_line();
    let t = sc.get();
    return t;
}
