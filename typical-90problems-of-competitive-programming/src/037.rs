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

struct RangeMax {
    size: usize,
    dat: Vec<i64>,
}

impl RangeMax {
    fn new(sz: usize) -> Self {
        let mut size_ = 1;
        while size_ <= sz {
            size_ *= 2;
        }
        RangeMax {
            size: size_,
            dat: vec![-1_000_000_000_000_000_000; size_ * 2],
        }
    }
    fn update(&mut self, mut pos: usize, x: i64) {
        pos += self.size;
        self.dat[pos] = x;
        while pos >= 2 {
            pos >>= 1;
            self.dat[pos as usize] =
                i64::max(self.dat[2 * pos as usize], self.dat[2 * pos as usize + 1]);
        }
    }
    fn query_(&mut self, l: i64, r: i64, a: i64, b: i64, u: i64) -> i64 {
        if l <= a && b <= r {
            return self.dat[u as usize];
        }
        if r <= a || b <= l {
            return -1_000_000_000_000_000_000;
        }
        let v1 = self.query_(l, r, a, (a + b) >> 1, u * 2);
        let v2 = self.query_(l, r, (a + b) >> 1, b, u * 2 + 1);
        return i64::max(v1, v2);
    }
    fn query(&mut self, l: i64, r: i64) -> i64 {
        return self.query_(l, r, 0, self.size as i64, 1);
    }
}

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);
    let lower = -1_000_000_000_000_000_000;

    sc.new_line();
    let W: usize = sc.get();
    let N: usize = sc.get();
    let mut L = vec![0; 1 << 18];
    let mut R = vec![0; 1 << 18];
    let mut V = vec![0; 1 << 18];
    let mut Z = Vec::new();
    let mut dp = vec![vec![lower; 10009]; 509];
    for i in 1..=N {
        sc.new_line();
        L[i] = sc.get();
        R[i] = sc.get();
        V[i] = sc.get();
    }
    for i in 0..510 {
        Z.push(RangeMax::new(W + 2));
    }
    dp[0][0] = 0;
    Z[0].update(0, 0);
    for i in 1..=N {
        for j in 0..=W {
            dp[i][j] = dp[i - 1][j];
        }
        for j in 0..=W {
            let cl = (j as i64 - R[i]).max(0);
            let cr = (j as i64 - L[i] + 1).max(0);
            if cl == cr {
                continue;
            }
            let val = Z[i - 1].query(cl, cr);
            if val != lower {
                dp[i][j] = dp[i][j].max(val + V[i]);
            }
        }
        for j in 0..=W {
            Z[i].update(j, dp[i][j]);
        }
    }
    if dp[N][W] == lower {
        println!("-1");
    } else {
        println!("{}", dp[N][W]);
    }
}
