use std::cmp::{max, min};
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

    let mut ans = -1_000_000_007;

    sc.new_line();
    let n: usize = sc.get();
    let mut v: Vec<i64> = Vec::new();
    for _ in 0..n {
        sc.new_line();
        let a: i64 = sc.get();
        v.push(a);
    }
    let mut a = v.clone();
    let mut b = v.clone();
    for i in 0..a.len() - 1 {
        if a[i + 1] > a[i] {
            a[i + 1] = a[i];
        }
    }
    for i in (1..b.len()).rev() {
        if b[i] > b[i - 1] {
            b[i - 1] = b[i];
        }
    }
    for i in 0..n - 1 {
        ans = max(ans, b[i + 1] - a[i]);
    }

    println!("{}", ans);
}

fn print_vec(v: &Vec<i64>) {
    for i in 0..v.len() - 1 {
        print!("{} ", v[i]);
    }
    println!("{}", v[v.len() - 1]);
}
