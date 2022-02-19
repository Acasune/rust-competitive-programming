use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::vec;
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

struct SegTree {
    n: usize,
    dat: Vec<i64>,
    lazy_dat: Vec<i64>,
}

impl SegTree {
    fn new(n: usize) -> Self {
        let mut n_ = 1;
        while n_ < n {
            n_ *= 2;
        }
        SegTree {
            n: n_,
            dat: vec![0; n_ * 2],
            lazy_dat: vec![0; n_ * 2],
        }
    }
    fn push(&mut self, k: usize) {
        if k < self.n {
            self.lazy_dat[k * 2] = self.lazy_dat[k * 2].max(self.lazy_dat[k]);
            self.lazy_dat[k * 2 + 1] = self.lazy_dat[k * 2 + 1].max(self.lazy_dat[k]);
        }
        self.dat[k] = self.dat[k].max(self.lazy_dat[k]);
        self.lazy_dat[k] = 0;
    }
    fn update(&mut self, a: usize, b: usize, x: i64, k: usize, l: usize, r: usize) {
        self.push(k);
        if r <= a || b <= l {
            return;
        }
        if a <= l && r <= b {
            self.lazy_dat[k] = x;
            self.push(k);
            return;
        }
        self.update(a, b, x, k * 2, l, (l + r) >> 1);
        self.update(a, b, x, k * 2 + 1, (l + r) >> 1, r);
        self.dat[k] = self.dat[k * 2].max(self.dat[k * 2 + 1]);
        return;
    }
    fn query(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
        self.push(k);
        if r <= a || b <= l {
            return 0;
        }
        if a <= l && r <= b {
            return self.dat[k];
        }
        let vl = self.query(a, b, k * 2, l, (l + r) >> 1);
        let vr = self.query(a, b, k * 2 + 1, (l + r) >> 1, r);
        return vl.max(vr);
    }
}

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let W = sc.get::<usize>();
    let N = sc.get::<usize>();

    let mut segtree = SegTree::new(W);
    let mut ans = Vec::<i64>::new();
    for _ in 0..N {
        sc.new_line();
        let l = sc.get::<usize>();
        let r = sc.get::<usize>();
        let x = segtree.query(l - 1, r, 1, 0, segtree.n) + 1;
        segtree.update(l - 1, r, x, 1, 0, segtree.n);
        ans.push(x);
    }
    for x in ans {
        println!("{}", x);
    }
}
