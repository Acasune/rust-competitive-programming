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
    let N: usize = sc.get();
    let S: usize = sc.get();
    let mut ab = vec![vec![1, 2]; N + 1];

    for i in 1..=N {
        sc.new_line();
        ab[i] = sc.get_as_vec::<i64>();
    }
    let mut dp = vec![vec![false; S + 1]; N + 1];
    dp[0][0] = true;
    for i in 1..=N {
        let tmp = ab[i].clone();
        let a = tmp[0];
        let b = tmp[1];
        for s in 0..=S {
            if s as i64 - a >= 0 && dp[i - 1][s - a as usize] {
                dp[i][s] = true;
            }
            if s as i64 - b >= 0 && dp[i - 1][s - b as usize] {
                dp[i][s] = true;
            }
        }
    }
    if !dp[N][S] {
        println!("Impossible");
        return;
    }
    let mut vec = vec!['A'; N];
    let mut next = S;
    for i in (0..N).rev() {
        let tmp = ab[i + 1].clone();
        let a = tmp[0];
        let b = tmp[1];
        if (next >= a as usize) && dp[i][next - a as usize] {
            vec[i] = 'A';
            next = next - a as usize;
        } else {
            vec[i] = 'B';
            next = next - b as usize;
        }
    }
    println!("{}", vec.iter().collect::<String>());
}
