use itertools::Itertools;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{i64, io::*, str::FromStr, usize};

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
    let N = sc.get::<usize>();
    sc.new_line();
    let nodes = sc
        .get_as_vec::<char>()
        .iter()
        .map(|&a| if a == 'a' { 0 as usize } else { 1 as usize })
        .collect_vec();
    let mut edges = vec![Vec::<usize>::new(); N];

    for i in 1..N {
        sc.new_line();
        let a = sc.get::<usize>() - 1;
        let b = sc.get::<usize>() - 1;
        edges[a].push(b);
        edges[b].push(a);
    }
    let mut dp = vec![vec![0; 3]; N];
    rec(0, &mut dp, 1_000_000_000_000, &nodes, &edges, 1_000_000_007);
    println!("{}", dp[0][2]);
}

fn rec(
    nd: usize,
    dp: &mut Vec<Vec<i64>>,
    pre: usize,
    nodes: &Vec<usize>,
    edges: &Vec<Vec<usize>>,
    md: i64,
) {
    if edges[nd].is_empty() {
        let kind = nodes[nd];
        dp[nd][kind] += 1;
        return;
    }
    let mut pow1 = 1;
    let mut pow2 = 1;
    for i in 0..edges[nd].len() {
        let next_node = edges[nd][i];
        if next_node == pre {
            continue;
        }
        rec(next_node, dp, nd, nodes, edges, md);
        if nodes[nd] == 0 {
            pow1 *= dp[next_node][0] + dp[next_node][2];
            pow2 *= dp[next_node][0] + dp[next_node][1] + 2 * dp[next_node][2];
        } else {
            pow1 *= dp[next_node][1] + dp[next_node][2];
            pow2 *= dp[next_node][0] + dp[next_node][1] + 2 * dp[next_node][2];
        }

        pow1 %= md;
        pow2 %= md;
    }
    if nodes[nd] == 0 {
        dp[nd][0] = pow1;
        dp[nd][2] = (md + pow2 - pow1) % md;
    } else {
        dp[nd][1] = pow1;
        dp[nd][2] = (md + pow2 - pow1) % md;
    }
}
