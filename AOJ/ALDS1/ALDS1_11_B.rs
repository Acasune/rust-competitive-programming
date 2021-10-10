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

fn dfs(start: usize, cnt: &mut usize, graph: &Vec<Vec<usize>>, ans: &mut Vec<Vec<usize>>) {
    ans[start][0] = *cnt;
    *cnt += 1;
    for e in &graph[start] {
        if ans[*e][0] > 0 {
            continue;
        }
        dfs(*e, cnt, graph, ans);
    }

    ans[start][1] = *cnt;
    *cnt += 1;
}

#[allow(unused_variables)]
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n = sc.get::<usize>();
    let mut graph = vec![Vec::<usize>::new(); n];
    let mut ans = vec![vec![0; 2]; n];

    for _ in 0..n {
        sc.new_line();
        let u = sc.get::<usize>() - 1;
        let k = sc.get::<usize>();
        for i in 0..k {
            let a = sc.get::<usize>() - 1;
            graph[u].push(a);
        }
    }
    let mut cnt = 1;
    for i in 0..n {
        if ans[i][0] > 0 {
            continue;
        }
        dfs(i, &mut cnt, &graph, &mut ans);
    }

    for i in 0..n {
        println!("{} {} {}", i + 1, ans[i][0], ans[i][1]);
    }
}
