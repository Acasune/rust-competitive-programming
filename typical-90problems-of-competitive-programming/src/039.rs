use itertools::Itertools;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{i64, io::*, str::FromStr};

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

fn dfs(dp: &mut Vec<i64>, pos: usize, pre: usize, G: &Vec<Vec<usize>>) {
    dp[pos] = 1;
    for e in &G[pos] {
        if *e == pre {
            continue;
        }
        dfs(dp, *e, pos, G);
        dp[pos] += dp[*e];
    }
}

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let N: usize = sc.get();
    let mut al = vec![0; N + 1];
    let mut bl = vec![0; N + 1];
    let mut G = vec![Vec::<usize>::new(); N + 1];
    let mut dp = vec![0; N + 1];
    for i in 1..N {
        sc.new_line();
        al[i] = sc.get::<usize>();
        bl[i] = sc.get::<usize>();
        G[al[i]].push(bl[i]);
        G[bl[i]].push(al[i]);
    }
    dfs(&mut dp, 1, 0, &G);
    let mut ans = 0;
    for i in 1..N {
        let r = dp[al[i]].min(dp[bl[i]]) as usize;
        ans += (N - r) * r;
    }
    println!("{}", ans);
}
