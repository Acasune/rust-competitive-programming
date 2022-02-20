use proconio::input;
use std::iter::FromIterator;
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
    input! {
      N:usize,X:usize,
      AB: [(usize,usize);N],
    }
    if N == 1 {
        if AB[0].0 == X || AB[0].1 == X {
            println!("{}", "Yes");
        } else {
            println!("{}", "No");
        }
        return;
    }

    let mut dp = vec![false; X + 10];
    dp[0] = true;
    for i in 0..N - 1 {
        let a = AB[i].0;
        let b = AB[i].1;
        let mut memo = Vec::<(usize, usize)>::new();
        for j in (0..X).rev() {
            if j + a < X && dp[j] {
                memo.push((j, a));
            }
            if j + b < X && dp[j] {
                memo.push((j, b));
            }
        }
        dp = vec![false; X + 10];

        for &(j, a) in &memo {
            dp[j + a] = true;
        }
    }
    // println!("{:?}", dp);
    if (AB[N - 1].0 < X && dp[X - AB[N - 1].0]) || (AB[N - 1].1 < X && dp[X - AB[N - 1].1]) {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}
