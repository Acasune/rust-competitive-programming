use proconio::input;
use std::collections::VecDeque;
use std::iter::FromIterator;
use std::{
    cmp::{max, min},
    collections::HashSet,
    io::*,
    str::FromStr,
};

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
    input! {
      N:usize,
      A:[usize;N]
    }
    let mut deq = VecDeque::<usize>::new();
    let mut deq2 = VecDeque::<(usize, usize)>::new();
    deq.push_back(1);
    deq2.push_back((1, 2));

    for &a in &A {
        let x = deq.pop_back().unwrap();
        let (y, z) = deq2.pop_back().unwrap();
        if x != a {
            deq.push_back(x);
            deq.push_back(a);
            deq2.push_back((y, z));
            deq2.push_back((a, 1));
        } else {
            if a == z + 1 {
                for i in 0..a - 2 {
                    deq.pop_back();
                }
            } else {
                deq.push_back(x);
                deq.push_back(a);
                deq2.push_back((y, z + 1));
            }
        }
        println!("{}", deq.len() - 1);
    }
}
