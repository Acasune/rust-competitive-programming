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

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let mut N = sc.get::<usize>();
    let mut K = sc.get::<usize>();
    let md = 1_000_000_007;

    // K<= 2 || N<=2
    if K == 2 {
        if N <= 2 {
            println!("2");
        } else {
            println!("0");
        }
        return;
    } else if K == 1 {
        if N == 1 {
            println!("1");
        } else {
            println!("0");
        }
        return;
    } else if N == 1 {
        println!("{}", K);
        return;
    } else if N == 2 {
        println!("{}", K * (K - 1));
        return;
    }

    let mut ret = K * (K - 1) % md;
    K -= 2;
    N -= 2;
    while N > 0 {
        if N & 1 == 1 {
            ret *= K;
            ret %= md;
        }
        N >>= 1;
        K *= K;
        K %= md;
    }
    println!("{}", ret);
}
