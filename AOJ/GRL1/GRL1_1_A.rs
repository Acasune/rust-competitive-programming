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
    let n_v: usize = sc.get();
    let n_e: usize = sc.get();
    let s: usize = sc.get();
    let INF = usize::MAX / 10;
    let mut E = vec![vec![]; n_v];
    for _ in 0..n_e {
        sc.new_line();
        let u: usize = sc.get();
        let v: usize = sc.get();
        let c: usize = sc.get();
        let dt: dist_to = (v, c);
        E[u].push(dt);
    }

    let dists = dijkstra(s, &E);
    dists.into_iter().for_each(|x| {
        if x == INF {
            println!("{}", "INF");
        } else {
            println!("{}", x);
        }
    });
}
fn dijkstra(s: usize, E: &Vec<Vec<(usize, usize)>>) -> Vec<usize> {
    let N = E.len();
    let mut dists = vec![usize::MAX / 10; N];
    dists[s] = 0;
    let mut que = BinaryHeap::<Reverse<(usize, usize)>>::new();
    que.push(Reverse((0, s)));
    while let Some(Reverse((c, s))) = que.pop() {
        if dists[s] < c {
            continue;
        }
        for &(nxt, dist) in &E[s] {
            if dists[nxt] > dists[s] + dist {
                dists[nxt] = dists[s] + dist;
                que.push(Reverse((dist, nxt)));
            }
        }
    }
    return dists;
}
