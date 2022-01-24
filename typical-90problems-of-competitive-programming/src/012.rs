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

struct UnionFind {
    pars: Vec<i64>,
}

impl UnionFind {
    fn new(W: usize, H: usize) -> Self {
        UnionFind {
            pars: vec![-1; W * H],
        }
    }
    fn root(&mut self, hash: i64) -> i64 {
        if self.pars[hash as usize] == -1 {
            return hash;
        }
        self.pars[hash as usize] = self.root(self.pars[hash as usize]);
        return self.pars[hash as usize];
    }
    fn unite(&mut self, hash1: i64, hash2: i64) {
        let u = self.root(hash1);
        let v = self.root(hash2);
        if u == v {
            return;
        }
        self.pars[u as usize] = v;
    }
    fn isSame(&mut self, hash1: i64, hash2: i64) -> bool {
        self.root(hash1) == self.root(hash2)
    }
}

fn query1(py: usize, px: usize, visited: &mut Vec<Vec<bool>>, uf: &mut UnionFind) {
    if visited[py][px] {
        return;
    }
    let dy: [i64; 4] = [-1, 0, 1, 0];
    let dx: [i64; 4] = [0, -1, 0, 1];
    let H: i64 = visited.len() as i64;
    let W: i64 = visited[0].len() as i64;
    for i in 0..4 {
        let ny: i64 = py as i64 + dy[i];
        let nx: i64 = px as i64 + dx[i];
        if visited[ny as usize][nx as usize] {
            let hash1: i64 = py as i64 * W + px as i64;
            let hash2: i64 = ny * W + nx;
            uf.unite(hash1, hash2);
        }
    }
    visited[py as usize][px as usize] = true;
}
fn query2(
    py: usize,
    px: usize,
    qy: usize,
    qx: usize,
    visited: &mut Vec<Vec<bool>>,
    uf: &mut UnionFind,
) -> bool {
    if !visited[py][px] || !visited[qy][qx] {
        return false;
    }
    let W: i64 = visited[0].len() as i64;
    let hash1: i64 = py as i64 * W + px as i64;
    let hash2: i64 = qy as i64 * W + qx as i64;
    return uf.isSame(hash1, hash2);
}

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let H: usize = sc.get();
    let W: usize = sc.get();
    let mut visited = vec![vec![false; W + 10]; H + 10];
    let mut uf = UnionFind::new(W + 10, H + 10);
    sc.new_line();
    let Q: usize = sc.get();
    for _ in 0..Q {
        sc.new_line();
        let t: usize = sc.get();
        if t == 1 {
            let r: usize = sc.get();
            let c: usize = sc.get();
            query1(r, c, &mut visited, &mut uf);
        } else {
            let ra: usize = sc.get();
            let ca: usize = sc.get();
            let rb: usize = sc.get();
            let cb: usize = sc.get();
            if query2(ra, ca, rb, cb, &mut visited, &mut uf) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
