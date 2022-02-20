use proconio::input;
use std::iter::FromIterator;
use std::{
    cmp::{max, min},
    collections::HashSet,
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
    input! {
      N:usize,
      uv:[(usize,usize);N-1],
    }
    let mut edges = vec![Vec::<usize>::new(); N + 1];
    for &(u, v) in &uv {
        edges[u].push(v);
        edges[v].push(u);
    }
    let mut memo = vec![(0, 0); N + 1];
    dfs(1, &edges, &mut memo, 1_000_000_000, 1);
    for &(l, r) in &memo[1..] {
        println!("{} {}", l, r);
    }
    // println!("{:?}", memo);
}
fn dfs(
    root: usize,
    edges: &Vec<Vec<usize>>,
    memo: &mut Vec<(usize, usize)>,
    pre: usize,
    base: usize,
) -> usize {
    if root != 1 && edges[root].len() == 1 {
        return 1;
    }
    let mut child = base;
    for &edge in &edges[root] {
        if pre == edge {
            continue;
        }
        let ret = dfs(edge, &edges, memo, root, child);
        memo[edge].0 = child;
        memo[edge].1 = child + ret - 1;
        child += ret;
    }

    memo[root].0 = base;
    memo[root].1 = child - 1;
    child - base
}
