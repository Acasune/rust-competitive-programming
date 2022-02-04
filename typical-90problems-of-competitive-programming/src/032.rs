use itertools::Itertools;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashSet, VecDeque};
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

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let N: usize = sc.get();
    let mut costs = Vec::<Vec<usize>>::new();
    for _ in 0..N {
        sc.new_line();
        let a = sc.get_as_vec::<usize>();
        costs.push(a);
    }
    sc.new_line();
    let M: usize = sc.get();
    let mut bad_pairs = HashSet::new();
    for _ in 0..M {
        sc.new_line();
        let a: usize = sc.get();
        let b: usize = sc.get();
        bad_pairs.insert((a - 1, b - 1));
    }
    let mut min_cost = std::usize::MAX;

    for route in (0..N).permutations(N) {
        if route
            .windows(2)
            .any(|w| bad_pairs.contains(&(w[0], w[1])) || bad_pairs.contains(&(w[1], w[0])))
        {
            continue;
        }
        let cost: usize = route.iter().enumerate().map(|(i, j)| costs[*j][i]).sum();
        min_cost = min_cost.min(cost);
    }
    if min_cost == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", min_cost);
    }
}
