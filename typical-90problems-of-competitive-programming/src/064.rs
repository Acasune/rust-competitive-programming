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
    let mut Q: usize = sc.get();
    sc.new_line();
    let a = sc.get_as_vec::<i64>();
    let mut diffs = vec![0 as i64; N * 2];
    let mut ret = 0;
    for i in 1..N {
        diffs[i] = a[i] - a[i - 1];
        ret += i64::abs(diffs[i]);
    }
    for q in 0..Q {
        sc.new_line();
        let q = sc.get_as_vec::<i64>();
        let l = q[0] as usize;
        let r = q[1] as usize;
        let v = q[2];
        let before = i64::abs(diffs[l - 1]) + i64::abs(diffs[r]);
        if l >= 2 {
            diffs[l - 1] += v;
        }
        if r <= N - 1 {
            diffs[r] -= v;
        }
        let after = i64::abs(diffs[l - 1]) + i64::abs(diffs[r]);
        ret += after - before;
        println!("{}", ret);
    }
}
