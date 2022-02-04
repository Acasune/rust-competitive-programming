use itertools::Itertools;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{i64, io::*, str::FromStr};

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
    let Q: usize = sc.get();
    let mut X = vec![0; N];
    let mut Y = vec![0; N];
    let mut max_x = i64::MIN;
    let mut max_y = i64::MIN;
    let mut min_x = i64::MAX;
    let mut min_y = i64::MAX;
    for i in 0..N {
        sc.new_line();
        let x: i64 = sc.get();
        let y: i64 = sc.get();
        let p1 = y + x;
        let p2 = y - x;
        X[i] = p1;
        Y[i] = p2;
        max_x = max_x.max(p1);
        min_x = min_x.min(p1);
        max_y = max_y.max(p2);
        min_y = min_y.min(p2);
    }
    for _ in 0..Q {
        sc.new_line();
        let q: usize = sc.get();
        println!(
            "{}",
            i64::max(
                i64::max(i64::abs(max_x - X[q - 1]), i64::abs(min_x - X[q - 1])),
                i64::max(i64::abs(max_y - Y[q - 1]), i64::abs(min_y - Y[q - 1]))
            )
        );
    }
}
