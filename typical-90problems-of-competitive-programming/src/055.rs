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

fn dfs(
    ans: &mut usize,
    idx: usize,
    size: usize,
    production: usize,
    P: usize,
    Q: usize,
    al: &Vec<usize>,
) {
    if size == 5 {
        if production % P == Q {
            *ans += 1;
        }
        return;
    }
    if idx >= al.len() {
        return;
    }
    dfs(ans, idx + 1, size + 1, (production * al[idx]) % P, P, Q, al);
    dfs(ans, idx + 1, size, production % P, P, Q, al);
}

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let N: usize = sc.get();
    let P: usize = sc.get();
    let Q: usize = sc.get();
    sc.new_line();
    let al = sc.get_as_vec::<usize>();
    let mut ret: usize = 0;
    dfs(&mut ret, 0, 0, 1, P, Q, &al);
    println!("{}", ret);
}
