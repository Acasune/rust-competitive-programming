#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    cell::RefCell,
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

#[allow(unused_variables)]
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n = sc.get::<usize>();
    let mut graph = vec![Vec::<usize>::new(); n];
    let mut ans = vec![-1; n];

    for _ in 0..n {
        sc.new_line();
        let u = sc.get::<usize>() - 1;
        let k = sc.get::<usize>();
        for i in 0..k {
            let a = sc.get::<usize>() - 1;
            graph[u].push(a);
        }
    }
    let mut que = VecDeque::<(usize, usize)>::new();
    for e in &graph[0] {
        que.push_back((0, *e));
    }
    ans[0] = 0;

    while !que.is_empty() {
        let (from, to) = que.pop_front().unwrap();
        if ans[to] != -1 {
            continue;
        }
        ans[to] = ans[from] + 1;
        for e in &graph[to] {
            que.push_back((to, *e));
        }
    }

    for i in 0..n {
        println!("{} {}", i + 1, ans[i]);
    }
}
