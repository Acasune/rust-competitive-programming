use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashSet, VecDeque};
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
    let H = 1_001;
    let W = 1_001;

    let mut field = vec![vec![0 as i64; W]; H];
    let mut ans = vec![0; N + 1];

    for i in 0..N {
        sc.new_line();
        let x1: usize = sc.get();
        let y1: usize = sc.get();
        let x2: usize = sc.get();
        let y2: usize = sc.get();
        field[y1][x1] += 1;
        field[y1][x2] -= 1;
        field[y2][x1] -= 1;
        field[y2][x2] += 1;
    }
    for j in 0..H {
        for i in 1..W {
            field[j][i] += field[j][i - 1];
        }
    }
    for i in 0..W {
        for j in 1..H {
            field[j][i] += field[j - 1][i];
        }
    }
    for i in 0..W {
        for j in 0..H {
            ans[field[j][i] as usize] += 1;
        }
    }
    for i in 1..N + 1 {
        println!("{}", ans[i]);
    }
}
