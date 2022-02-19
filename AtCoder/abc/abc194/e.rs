use std::{
    cmp::{max, min, Reverse},
    collections::{BinaryHeap, HashSet},
    io::*,
    str::FromStr,
};
use std::{collections::hash_set, iter::FromIterator};

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
    let M = sc.get::<usize>();

    sc.new_line();
    let al = sc.get_as_vec::<usize>();
    let mut st = HashSet::<usize>::new();
    let mut count = vec![0; N + 1];
    for i in 0..M {
        count[al[i]] += 1;
    }
    let mut found = count.iter().map(|&b| b == 0).collect::<Vec<bool>>();
    for i in M..N {
        count[al[i]] += 1;
        count[al[i - M]] -= 1;
        found[al[i - M]] |= count[al[i - M]] == 0;
    }
    println!("{}", found.iter().position(|&b| b).unwrap());
}
