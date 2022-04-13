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
    sc.new_line();
    let mut vec: Vec<i64> = sc.get_as_vec();

    let mut tree = FenwickTree::<i64>::new(n + 1, 0);
    let mut ans = 0i64;
    vec = compression(vec);
    for i in 0..n {
        ans += i as i64 - tree.sum(vec[i] as usize);
        tree.add(vec[i] as usize, 1);
    }
    println!("{}", ans);
}

fn compression(v: Vec<i64>) -> Vec<i64> {
    let n = v.len();
    let mut v2 = v.clone();
    v2.sort();
    let mut v_c = vec![0i64; n];
    for i in 0..n {
        let t = v[i];
        let idx = v2
            .binary_search_by_key(&(2 * t), |&a| 2 * a + 1)
            .err()
            .unwrap();
        v_c[i] = idx as i64 + 1;
    }

    v_c
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
