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
    let N = sc.get::<usize>();
    let K = sc.get::<usize>();
    let mut memo = vec![vec![0; 5_010 + K]; 5_010 + K];
    for i in 0..N {
        sc.new_line();
        let a = sc.get::<usize>();
        let b = sc.get::<usize>();
        memo[a][b] += 1;
    }

    for j in 1..=5000 {
        for i in 0..=(5000) {
            memo[j][i] += memo[j - 1][i];
        }
    }
    for i in 1..=5000 {
        for j in 0..=5000 {
            memo[j][i] += memo[j][i - 1];
        }
    }
    let mut ans = 0;
    for i in 1..=5000 - K {
        for j in 1..=5000 - K {
            ans = ans.max(
                memo[j + K][i + K] - memo[j - 1][i + K] - memo[j + K][i - 1] + memo[j - 1][i - 1],
            );
        }
    }
    println!("{}", ans);
}
