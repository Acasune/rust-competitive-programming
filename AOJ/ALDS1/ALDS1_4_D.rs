#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    cell::RefCell,
    collections::HashSet,
    collections::VecDeque,
    fmt::Display,
    rc::{Rc, Weak},
};
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

fn isLoadable(v: &Vec<i64>, p: i64, k: usize) -> bool {
    let mut num = 1;
    let mut total = 0;
    for ev in v {
        if total + ev <= p {
            total += ev;
        } else {
            if ev > &p {
                return false;
            }

            num += 1;
            total = *ev;
        }
    }
    return num <= k;
}

#[allow(unused_variables)]
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    let mut st = HashSet::<String>::new();

    sc.new_line();
    let n: usize = sc.get();
    let k: usize = sc.get();
    let mut v: Vec<i64> = sc.get_as_vec();
    let mut sum = 0;

    for _ in 0..n {
        sc.new_line();
        let a: i64 = sc.get();
        v.push(a);
        sum += a;
    }
    let mut left = 0;
    let mut right = sum;
    let mut mid;
    let mut ans = sum;

    while left < right - 1 {
        mid = (right + left) / 2;
        if isLoadable(&v, mid, k) {
            ans = min(ans, mid);
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("{}", ans);
}
