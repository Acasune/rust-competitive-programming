﻿#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    cell::RefCell,
    collections::BinaryHeap,
    collections::HashMap,
    collections::VecDeque,
    fmt::Display,
    rc::{Rc, Weak},
};
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
    pub fn new(reader: R) -> Scanner<R> {
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

#[allow(unused_variables)]
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n = sc.get::<usize>();
    let mut graph = vec![vec![1_000_000_100; n]; n];
    for _ in 0..n {
        sc.new_line();
        let a = sc.get::<usize>();
        let k = sc.get::<usize>();
        for i in 0..k {
            let v = sc.get::<usize>();
            let c = sc.get::<i32>();
            graph[a][v] = c;
        }
    }
    let mut dist = vec![1_000_000_100; n];
    dist[0] = 0;

    let mut heap = BinaryHeap::<(i32, i32)>::new();
    heap.push((0, 0));
    dist[0] = 0;

    while !heap.is_empty() {
        let (cost, v) = heap.pop().unwrap();
        if dist[v as usize] < -cost {
            continue;
        }
        for i in 0..graph[v as usize].len() {
            let c = graph[v as usize][i];

            if dist[i as usize] > c + dist[v as usize] {
                dist[i as usize] = c + dist[v as usize];
                heap.push((-c, i as i32));
            }
        }
    }
    for (i, c) in dist.iter().enumerate() {
        println!("{} {}", i, c);
    }
}
