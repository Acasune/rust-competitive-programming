use itertools::Itertools;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{i64, io::*, str::FromStr, usize};

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
    let mut N: usize = sc.get();
    sc.new_line();
    let mut al = sc.get_as_vec::<usize>();

    let inf = 100_000_000_000;
    let mut dp = vec![inf; N];
    let mut dp_len = vec![inf; N];
    let mut dp_rev = vec![inf; N];
    let mut dp_rev_len = vec![inf; N];
    for i in 0..N {
        let idx = dp
            .binary_search_by_key(&(al[i] * 2), |b| 2 * b + 1)
            .err()
            .unwrap();
        dp[idx] = al[i];
        dp_len[i] = idx + 1;
    }
    for i in (0..N).rev() {
        let idx = dp_rev
            .binary_search_by_key(&(al[i] * 2), |b| 2 * b + 1)
            .err()
            .unwrap();
        dp_rev[idx] = al[i];
        dp_rev_len[i] = idx + 1;
    }

    let mut ret = 0;
    for i in 0..N {
        ret = max(ret, dp_len[i] + dp_rev_len[i] - 1);
    }
    println!("{}", ret);
}
