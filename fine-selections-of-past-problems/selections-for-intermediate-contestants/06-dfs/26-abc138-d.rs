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

type edges = Vec<Vec<usize>>;

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n = sc.get::<usize>();
    let q = sc.get::<usize>();

    let mut cnts = vec![0; n];
    let mut visited = vec![false; n];
    let mut edges = vec![Vec::new(); n];
    for i in 0..n - 1 {
        sc.new_line();
        let a = sc.get::<usize>() - 1;
        let b = sc.get::<usize>() - 1;
        edges[a].push(b);
        edges[b].push(a);
    }
    for j in 0..q {
        sc.new_line();
        let a = sc.get::<usize>() - 1;
        let b = sc.get::<usize>();
        cnts[a] += b;
    }

    visited[0] = true;
    for &e in &edges[0] {
        dfs(0, e, &mut visited, &edges, &mut cnts);
    }
    for i in 0..n {
        if i == 0 {
            print!("{}", cnts[i])
        } else {
            print!(" {}", cnts[i])
        }
    }
    println!("")
}

fn dfs(prev: usize, now: usize, visited: &mut Vec<bool>, edges: &edges, cnts: &mut Vec<usize>) {
    visited[now] = true;
    cnts[now] += cnts[prev];
    for &e in &edges[now] {
        if visited[e] {
            continue;
        }
        dfs(now, e, visited, &edges, cnts)
    }
}
