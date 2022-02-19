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
    let N = sc.get::<usize>();
    sc.new_line();
    let mut al = sc.get_as_vec::<i64>();
    sc.new_line();
    let mut bl = sc.get_as_vec::<i64>();
    sc.new_line();
    let mut cl = sc.get_as_vec::<i64>();
    let mut A = vec![0; 46];
    let mut B = vec![0; 46];
    let mut C = vec![0; 46];

    for i in 0..N {
        let a = al[i];
        let b = bl[i];
        let c = cl[i];

        A[(a % 46) as usize] += 1;
        B[(b % 46) as usize] += 1;
        C[(c % 46) as usize] += 1;
    }
    let mut ans = 0i64;
    for i in 0..46 {
        for j in 0..46 {
            for k in 0..46 {
                if (i + j + k) % 46 != 0 {
                    continue;
                }
                ans += A[i] * B[j] * C[k];
            }
        }
    }
    println!("{}", ans);
}
