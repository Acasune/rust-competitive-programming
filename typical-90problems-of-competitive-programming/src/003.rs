use std::cmp::{max, min};
use std::collections::VecDeque;
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
    let mut edges = vec![Vec::<usize>::new(); N];
    for _ in 0..N - 1 {
        sc.new_line();
        let a: usize = sc.get();
        let b: usize = sc.get();
        edges[a - 1].push(b - 1);
        edges[b - 1].push(a - 1);
    }

    let memo = bfs(0, &mut edges);
    let mut mx = -1;
    let mut idx: usize = 0;
    for i in 1..N {
        if memo[i] > mx {
            mx = memo[i];
            idx = i;
        }
    }
    mx = -1;
    let memo2 = bfs(idx, &mut edges);
    for i in 0..N {
        if i == idx {
            continue;
        }

        if memo2[i] > mx {
            mx = memo2[i];
        }
    }
    println!("{}", mx + 1);
}

fn bfs(s: usize, edges: &mut Vec<Vec<usize>>) -> Vec<i64> {
    let mut memo = vec![-1; edges.len()];
    memo[s] = 0;
    let mut que = VecDeque::<usize>::new();
    que.push_back(s);
    while !que.is_empty() {
        let a = que.pop_front().unwrap();

        for b in edges[a].iter() {
            if memo[*b] == -1 {
                memo[*b] = memo[a] + 1;
                que.push_back(*b);
            }
        }
    }
    memo
}
