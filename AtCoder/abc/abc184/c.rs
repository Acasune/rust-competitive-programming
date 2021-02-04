use std::{
    cmp::{max, min},
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
    let r1 = sc.get::<i64>();
    let c1 = sc.get::<i64>();
    sc.new_line();
    let mut r2 = sc.get::<i64>();
    let mut c2 = sc.get::<i64>();
    r2 -= r1;
    c2 -= c1;

    if r2 == 0 && c2 == 0 {
        println!("0");
    } else if r2.abs() == c2.abs() {
        println!("1");
    } else if c2.abs() + r2.abs() <= 3 {
        println!("1");
    } else if c2 % 2 == r2 % 2 {
        println!("2");
    } else if max(r2.abs(), c2.abs()) - min(r2.abs(), c2.abs()) <= 3 {
        println!("2");
    } else if r2.abs() + c2.abs() <= 6 {
        println!("2");
    } else {
        println!("3");
    }
}
