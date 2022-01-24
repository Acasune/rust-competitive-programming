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

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let N: usize = sc.get();
    let mut g1 = vec![0; N + 1];
    let mut g2 = vec![0; N + 1];
    for i in 0..N {
        sc.new_line();
        let c = sc.get::<i64>();
        let p = sc.get::<i64>();
        if c == 1 {
            g1[i + 1] = p;
        } else {
            g2[i + 1] = p;
        }
    }
    for i in 1..=N {
        g1[i] = g1[i - 1] + g1[i];
        g2[i] = g2[i - 1] + g2[i];
    }
    sc.new_line();
    let Q: usize = sc.get();
    for _ in 0..Q {
        sc.new_line();
        let l = sc.get::<usize>();
        let r = sc.get::<usize>();
        println!("{} {}", g1[r] - g1[l - 1], g2[r] - g2[l - 1]);
    }
}
