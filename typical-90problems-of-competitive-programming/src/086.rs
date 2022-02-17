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
    let Q = sc.get::<usize>();
    let mut X = vec![0; 100];
    let mut Y = vec![0; 100];
    let mut Z = vec![0; 100];
    let mut W = vec![0; 100];
    let mut xs = Vec::<usize>::new();
    let mut ys = Vec::<usize>::new();
    let mut zs = Vec::<usize>::new();
    let mut ws = vec![0; 100];
    for q in 1..=Q {
        sc.new_line();
        X[q] = sc.get::<usize>();
        Y[q] = sc.get::<usize>();
        Z[q] = sc.get::<usize>();
        W[q] = sc.get::<usize>();
    }
    let mut ret = 1;
    let md = 1_000_000_007;
    for i in 0..60 {
        ws = vec![0; 100];
        for j in 1..=Q {
            ws[j] = (W[j] / (1 << i)) % 2;
        }
        ret *= bit_search(&X, &Y, &Z, &ws, N, Q);
        ret %= md;
    }
    println!("{}", ret);
}

fn bit_search(
    X: &Vec<usize>,
    Y: &Vec<usize>,
    Z: &Vec<usize>,
    W: &Vec<usize>,
    N: usize,
    Q: usize,
) -> i64 {
    let mut ret = 0;

    for i in 0..(1 << N) {
        let mut bits = vec![0; 100];
        for j in 0..N {
            bits[j + 1] = (i / (1 << j)) % 2;
        }
        let mut flg = true;
        for j in 1..=Q {
            if ((bits[X[j]] | bits[Y[j]]) | bits[Z[j]]) != W[j] {
                flg = false;
            }
        }
        if flg {
            ret += 1;
        }
    }

    return ret;
}
