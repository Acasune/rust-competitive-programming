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
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n = sc.get::<usize>();
    let m = sc.get::<usize>();
    let q = sc.get::<usize>();

    let mut cs = vec![vec![0 as i64; n]; n];

    for _ in 0..m {
        sc.new_line();
        let l = sc.get::<usize>();
        let r = sc.get::<usize>();
        cs[l - 1][r - 1] += 1;
    }
    for i in 0..n {
        for j in 0..n - 1 {
            cs[i][j + 1] += cs[i][j]
        }
    }
    for j in (0..n).rev() {
        for i in (1..n).rev() {
            cs[i - 1][j] += cs[i][j];
        }
    }

    for _ in 0..q {
        sc.new_line();
        let l = sc.get::<usize>();
        let r = sc.get::<usize>();

        println!("{}", cs[l - 1][r - 1]);
    }
}
