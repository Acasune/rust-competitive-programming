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
    let P = sc.get::<i64>();
    let K = sc.get::<i64>();
    let mut A = Vec::<Vec<i64>>::new();

    for i in 0..N {
        sc.new_line();
        A.push(sc.get_as_vec::<i64>());
    }
    let K1 = less_than_K(P, K, &A);
    let K2 = less_than_K(P, K - 1, &A);
    if K2 - K1 > 100_000_000_000 {
        println!("Infinity");
    } else {
        println!("{}", K2 - K1);
    }
}
fn less_than_K(P: i64, K: i64, A: &Vec<Vec<i64>>) -> i64 {
    let mut l = 0;
    let mut r = 1_000_000_000_000;
    let mut x = 1_000_000_000_000;
    while r - l > 1 {
        let m = (r - l) / 2 + l;
        let cnt = count(m, P, A);
        if cnt <= K {
            r = m;
            x = x.min(m);
        } else {
            l = m;
        }
    }

    return x;
}
fn count(m: i64, P: i64, A: &Vec<Vec<i64>>) -> i64 {
    let n = A.len();
    let mut dist = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            if A[i][j] != -1 {
                dist[i][j] = A[i][j];
            } else {
                dist[i][j] = m;
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }
    }
    let mut cnt = 0;
    for i in 0..n {
        for j in i + 1..n {
            if dist[i][j] <= P {
                cnt += 1;
            }
        }
    }
    return cnt;
}
