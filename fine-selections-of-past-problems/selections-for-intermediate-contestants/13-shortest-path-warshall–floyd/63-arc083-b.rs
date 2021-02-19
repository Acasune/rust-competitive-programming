﻿use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
    fmt::Binary,
    io::*,
    str::FromStr,
};

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
    let mut n = sc.get::<usize>();

    let mut roads = vec![];
    for i in 0..n {
        sc.new_line();
        let tmp = sc.get_as_vec::<i64>();
        roads.push(tmp);
    }
    let mut conn = vec![vec![true; n]; n];

    let mut total = 0;

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if i == j || j == k || k == i {
                    continue;
                } else {
                    if roads[i][j] > roads[i][k] + roads[k][j] {
                        println!("-1");
                        return;
                    } else if roads[i][j] == roads[i][k] + roads[k][j] {
                        if conn[i][j] {
                            conn[i][j] = false;
                            conn[j][i] = false;
                        }
                    }
                }
            }
        }
    }

    for i in 0..n {
        for j in i + 1..n {
            if conn[i][j] {
                total += roads[i][j];
            }
        }
    }

    println!("{}", total);
}
