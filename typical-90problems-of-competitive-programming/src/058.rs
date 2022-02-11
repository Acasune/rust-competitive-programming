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
    let mut K: usize = sc.get();
    let md = 100_000;
    let mut n_vec = vec![1_000_000; md + 1];
    let mut idx = 0;
    while idx < K {
        let mut a = N;
        while a > 0 {
            N += a % 10;
            a /= 10;
        }
        N %= md;
        idx += 1;
        if n_vec[N] != 1_000_000 {
            break;
        } else {
            n_vec[N] = idx;
        }
    }
    if idx == K {
        println!("{}", N);
        return;
    }
    K -= n_vec[N];
    K %= idx - n_vec[N];
    for _ in 0..K {
        let mut a = N;
        while a > 0 {
            N += a % 10;
            a /= 10;
        }
        N %= md;
    }
    println!("{}", N);
}
