﻿use std::{
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
    let n = sc.get::<usize>();
    let m = sc.get::<usize>();

    let mut d = vec![0];
    let mut c = vec![];

    for i in 0..n {
        sc.new_line();
        let tmp = sc.get::<usize>();
        d.push(tmp);
    }
    for i in 0..m {
        sc.new_line();
        let tmp = sc.get::<usize>();
        c.push(tmp);
    }

    let inf = 1_000_000_010;

    let mut dp = vec![vec![inf; m + 1]; n + 1];

    dp[0][0] = 0;

    for i in 0..n {
        for j in 0..m + 1 {
            for k in 1..m + 1 - j {
                dp[i + 1][j + k] = min(dp[i + 1][j + k], dp[i][j] + c[j + k - 1] * d[i + 1]);
            }
        }
    }
    let mut ans = inf;
    for j in 0..m + 1 {
        ans = min(ans, dp[n][j]);
    }
    println!("{}", ans);
}
