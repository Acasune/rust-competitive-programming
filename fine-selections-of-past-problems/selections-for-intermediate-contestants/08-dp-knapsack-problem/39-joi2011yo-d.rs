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

    let mut sec = vec![];

    sc.new_line();
    for i in 0..n - 1 {
        let a = sc.get::<usize>();
        sec.push(a);
    }
    let sm = sc.get::<usize>();

    let mut dp = vec![vec![0 as i128; 21]; n];
    dp[0][sec[0]] = 1;

    for i in 1..n - 1 {
        for j in 0..21 {
            if (j as i64 - sec[i] as i64) >= 0 && dp[i - 1][j - sec[i]] > 0 {
                dp[i][j] += dp[i - 1][j - sec[i]];
            }
            if (j + sec[i]) <= 20 && dp[i - 1][j + sec[i]] > 0 {
                dp[i][j] += dp[i - 1][j + sec[i]];
            }
        }
    }

    println!("{}", dp[n - 2][sm]);
}
