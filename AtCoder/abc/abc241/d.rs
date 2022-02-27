#![allow(unused_imports)]
#![allow(non_snake_case)]
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
use std::collections::BTreeSet;
use std::iter::FromIterator;
use std::{
    cmp::{max, min},
    collections::HashSet,
    f64,
    io::*,
    str::FromStr,
};

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
    let Q = sc.get::<usize>();
    let mut set = BTreeSet::new();
    for q in 0..Q {
        sc.new_line();
        let op: usize = sc.get();
        let x: i64 = sc.get();
        if op == 1 {
            set.insert((x, q));
        } else if op == 2 {
            let k: usize = sc.get();
            let ans = set.range(..(x, q)).rev().nth(k - 1).map_or(-1, |p| p.0);
            println!("{}", ans);
        } else if op == 3 {
            let k: usize = sc.get();
            let ans = set.range((x, 0)..).nth(k - 1).map_or(-1, |p| p.0);
            println!("{}", ans);
        }
    }
}
