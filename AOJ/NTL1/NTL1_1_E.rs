﻿use std::{
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
    type dist_to = (usize, usize);

    sc.new_line();
    let a: i64 = sc.get();
    let b: i64 = sc.get();
    let (mut x, mut y, g) = ext_gcd(a, b);
    if a == b && x > y {
        let tmp = x;
        x = y;
        y = tmp;
    }

    println!("{} {}", x, y);
}
fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (0, 1, b)
    } else {
        let (x, y, g) = ext_gcd(b % a, a);
        (y - b / a * x, x, g)
    }
}
