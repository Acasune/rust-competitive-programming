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
    sc.new_line();
    let mut arr = sc.get_as_vec::<i64>();
    arr.push(-100_000_000_000);
    arr.push(100_000_000_000);
    arr.sort();
    sc.new_line();
    let Q: usize = sc.get();
    for _ in 0..Q {
        sc.new_line();
        let b: i64 = sc.get();
        let idx = arr
            .binary_search_by_key(&(b * 2), |&a| 2 * a + 1)
            .err()
            .unwrap();
        println!(
            "{}",
            min(i64::abs(b - arr[idx]), i64::abs(b - arr[idx - 1]))
        );
    }
}
