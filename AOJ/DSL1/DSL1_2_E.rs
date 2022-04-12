use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
    fmt::Binary,
    io::*,
    str::FromStr,
};
use std::{f64, i64, usize};
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
    let n: usize = sc.get();
    let q: usize = sc.get();
    let mut tree = RangeAddQueryTree::new(n + 1);
    // basically 0 indexed
    for _ in 0..q {
        sc.new_line();
        let op: usize = sc.get();
        if op == 0 {
            let s: usize = sc.get();
            let t: usize = sc.get();
            let x: i64 = sc.get();
            tree.add(s - 1, t, x);
        } else {
            let pos: usize = sc.get();
            println!("{}", tree.sum(pos) - tree.sum(pos - 1));
        }
    }
}

struct RangeAddQueryTree {
    n: usize,
    bits: Vec<Vec<i64>>,
}

impl RangeAddQueryTree {
    fn new(n: usize) -> Self {
        let size = n;
        RangeAddQueryTree {
            n: size,
            bits: vec![vec![0; size]; 2],
        }
    }

    fn add_sub(&mut self, bit: usize, mut pos: usize, x: i64) {
        let mut cnt = 0;
        pos += 1;
        while pos <= self.n {
            self.bits[bit][pos - 1] += x;
            pos += pos & pos.wrapping_neg();
            cnt += 1;
        }
    }

    fn add(&mut self, l: usize, r: usize, x: i64) {
        self.add_sub(0, l, -x * (l as i64));
        self.add_sub(0, r, x * (r as i64));
        self.add_sub(1, l, x);
        self.add_sub(1, r, -x);
    }

    fn sum_sub(&self, bit: usize, mut pos: usize) -> i64 {
        let bits = &self.bits;
        let mut s = 0;
        while pos > 0 {
            s += bits[bit][pos - 1];
            pos -= pos & pos.wrapping_neg();
        }
        s
    }

    fn sum(&self, mut pos: usize) -> i64 {
        self.sum_sub(0, pos) + (self.sum_sub(1, pos) * pos as i64)
    }

    fn query(&self, l: usize, r: usize) -> i64 {
        self.sum(r - 1) - self.sum(l - 1)
    }
}
