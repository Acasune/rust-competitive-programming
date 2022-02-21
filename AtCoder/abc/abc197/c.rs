use proconio::input;
use std::iter::FromIterator;
use std::{
    cmp::{max, min},
    collections::HashSet,
    i64,
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
    input! {
        N:usize,
        A:[i64;N]
    }
    let mut ans = i64::MAX;
    for i in 0..(1 << N) {
        let mut cand = 0;
        let mut tmp = 0;
        let mut now = 0;
        for j in 0..N {
            if now == (i >> j) & 1 {
                tmp |= A[j];
            } else {
                cand ^= tmp;
                tmp = A[j];
                now = (i >> j) & 1;
            }
        }
        cand ^= tmp;
        ans = ans.min(cand);
    }
    println!("{}", ans);
}
