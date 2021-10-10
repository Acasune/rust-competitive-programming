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
    pub fn new(reader: R) -> Scanner<R> {
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

struct UnionFindTree {
    par: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFindTree {
    fn new(n: usize) -> Self {
        UnionFindTree {
            par: (0..n).collect::<Vec<usize>>(),
            rank: vec![0; n],
        }
    }
    fn find_root(&mut self, a: usize) -> usize {
        if self.par[a] == a {
            return a;
        }
        self.par[a] = self.find_root(self.par[a]);
        return self.par[a];
    }
    fn is_unite(&mut self, a: usize, b: usize) -> bool {
        return self.find_root(a) == self.find_root(b);
    }
    fn unite(&mut self, a: usize, b: usize) {
        let ra = self.find_root(a);
        let rb = self.find_root(b);
        if ra == rb {
            return;
        }
        if self.rank[ra] < self.rank[rb] {
            self.par[ra] = rb;
        } else {
            self.par[rb] = ra;
            if self.rank[ra] == self.rank[rb] {
                self.rank[ra] += 1;
            }
        }
    }
}

#[allow(unused_variables)]
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n = sc.get::<usize>();
    let m = sc.get::<usize>();

    let mut tree = UnionFindTree::new(n);

    for _ in 0..m {
        sc.new_line();
        let a = sc.get::<usize>();
        let b = sc.get::<usize>();
        tree.unite(a, b);
    }
    sc.new_line();
    let q = sc.get::<usize>();
    for _ in 0..q {
        sc.new_line();
        let a = sc.get::<usize>();
        let b = sc.get::<usize>();
        let s = if tree.is_unite(a, b) { "yes" } else { "no" };
        println!("{}", s);
    }
}
