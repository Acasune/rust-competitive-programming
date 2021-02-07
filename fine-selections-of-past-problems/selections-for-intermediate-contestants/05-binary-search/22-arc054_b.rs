// https://atcoder.jp/contests/joi2009ho/tasks/joi2009ho_b

use std::{
    cmp::{max, min},
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
    let p = sc.get::<f64>();

    let c = 0.0000000001;

    if fd(p, 0.) <= 0. {
        println!("0");
        return;
    }

    let mut l = 0.;
    let mut r = p;

    while r - l > c {
        let m = (r + l) / 2.;

        if fd(p, m) < 0. {
            l = m;
        } else {
            r = m;
        }
    }

    println!("{:.010}", l + p / 2.0_f64.powf(l / 1.5));
}

fn fd(p: f64, x: f64) -> f64 {
    let pow: f64 = 2.0_f64.powf(-1.0 / 1.5_f64);
    return 1.0 + p * pow.log(1.0_f64.exp()) * 2.0_f64.powf(-1.0 * (x / 1.5));
}
