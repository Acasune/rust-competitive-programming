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
    let mut tree = FenwickTree::<usize>::new(n + 1, 0);
    // basically 0 indexed
    for _ in 0..q {
        sc.new_line();
        let op: usize = sc.get();
        let a = sc.get::<usize>();
        let b = sc.get::<usize>();
        if op == 0 {
            tree.add(a - 1, b);
        } else {
            println!("{}", tree.sum(b) - tree.sum(a - 1));
        }
    }
}

struct FenwickTree<T> {
    n: usize,
    data: Vec<T>,
    e: T,
}

impl<T> FenwickTree<T>
where
    T: Clone,
    T: std::ops::AddAssign<T>,
{
    fn new(n: usize, e: T) -> Self {
        let size = n.next_power_of_two();
        FenwickTree {
            n: size,
            data: vec![e.clone(); size],
            e: e,
        }
    }
    fn add(&mut self, mut pos: usize, x: T) {
        pos += 1;
        while pos <= self.n {
            self.data[pos - 1] += x.clone();
            pos += pos & pos.wrapping_neg();
        }
    }
    fn sum(&self, mut pos: usize) -> T
    where
        T: std::ops::Add<Output = T>,
    {
        let data = &self.data;
        let mut s = self.e.clone();
        while pos > 0 {
            s += data[pos - 1].clone();
            pos -= pos & pos.wrapping_neg();
        }
        s += data[pos].clone();
        s
    }
}
