use std::{
    cmp::{max, min},
    collections::VecDeque,
    io::*,
    str::{Chars, FromStr},
    vec,
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
    sc.new_line();
    let a = sc.get_as_vec::<i64>();
    let mut to = vec![vec![]; n];
    for i in 0..m {
        sc.new_line();
        let t = sc.get_as_vec::<usize>();
        to[t[0] - 1].push(t[1] - 1);
    }

    let mut dp = vec![10000000000; n];

    for i in 0..n {
        for j in to[i].clone() {
            dp[j] = min(dp[j], min(dp[i], a[i]));
        }
    }
    let mut ans = -1_000_000_000;
    for i in 0..n {
        ans = max(ans, a[i] - dp[i]);
    }
    println!("{}", ans);
}
