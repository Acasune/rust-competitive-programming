use proconio::input;
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
        A:usize, B:usize,W:usize,
    }
    let WW = W * 1000;
    let mut dp1 = vec![1_000_000_000; WW + 1];
    let mut dp2 = vec![0; WW + 1];
    dp1[0] = 0;

    for i in 0..=WW {
        for j in A..=B {
            if i + j > WW {
                continue;
            }
            dp1[i + j] = dp1[i + j].min(dp1[i] + 1);
            dp2[i + j] = dp2[i + j].max(dp2[i] + 1);
        }
    }
    // println!("{:?}", dp1);
    if dp1[WW] == 1_000_000_000 {
        println!(
            "{}",
            "UNSATISFIABLE
        "
        );
    } else {
        println!("{} {}", dp1[WW], dp2[WW]);
    }
}
