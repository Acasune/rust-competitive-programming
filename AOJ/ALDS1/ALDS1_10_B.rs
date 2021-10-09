#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    cell::RefCell,
    collections::HashMap,
    collections::VecDeque,
    fmt::Display,
    rc::{Rc, Weak},
};
use std::{io::*, str::FromStr};

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

#[allow(unused_variables)]
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n = sc.get::<usize>();
    let mut v = Vec::<i64>::new();

    for i in 0..n {
        sc.new_line();
        let a = sc.get::<i64>();
        let b = sc.get::<i64>();
        if i == 0 {
            v.push(a);
        }
        v.push(b);
    }

    let mut memo = vec![vec![100_000_000_000; n + 1]; n + 1];
    for i in 0..n + 1 {
        memo[i][i] = 0;
    }
    for i in 1..n + 1 {
        //  col
        for j in 1..n + 1 - i {
            // row
            for k in 0..i {
                memo[j][i + j] = min(
                    memo[j][i + j],
                    memo[j][j + k] + memo[j + k + 1][i + j] + v[j - 1] * v[j + k] * v[i + j],
                );
            }
        }
    }
    println!("{}", memo[1][n]);
}
