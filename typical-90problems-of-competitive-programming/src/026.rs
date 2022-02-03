use std::cmp::{max, min, Reverse};
use std::collections::BinaryHeap;
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
        let a = sc.get::<usize>();
        let b = sc.get::<usize>();
        edges[a - 1].push(b - 1);
        edges[b - 1].push(a - 1);
    }
    let mut g1 = Vec::<usize>::new();
    let mut g2 = Vec::<usize>::new();
    let mut visited = vec![0; N];

    let mut que = VecDeque::new();
    que.push_back(0);
    visited[0] = 1;
    g1.push(0);
    while (!que.is_empty()) {
        let c = que.pop_front().unwrap();
        for i in 0..edges[c].len() {
            let d = edges[c][i];
            if visited[d] == 0 {
                if visited[c] == 1 {
                    visited[d] = 2;
                    g2.push(d);
                } else {
                    visited[d] = 1;
                    g1.push(d);
                }
                que.push_back(d);
            }
        }
    }
    if g1.len() < g2.len() {
        g1 = g2;
    }
    print!("{}", g1[0] + 1);
    for i in 1..N / 2 {
        print!(" {}", g1[i] + 1);
    }
    println!();
}
