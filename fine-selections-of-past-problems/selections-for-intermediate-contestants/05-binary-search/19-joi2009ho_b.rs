// https://atcoder.jp/contests/joi2009ho/tasks/joi2009ho_b

use std::{cmp::min, io::*, str::FromStr, usize};

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
    let d = sc.get::<usize>();
    sc.new_line();
    let n = sc.get::<usize>();
    sc.new_line();
    let m = sc.get::<usize>();

    let mut s = vec![0 as usize];
    s.push(d);

    for i in 0..n - 1 {
        sc.new_line();
        let k = sc.get::<usize>();
        s.push(k);
    }
    s.sort();
    let mut ans: i64 = 0;

    for i in 0..m {
        sc.new_line();
        let k = sc.get::<usize>();

        let idx = s
            .binary_search_by_key(&(2 * k), |a| 2 * a + 1)
            .err()
            .unwrap();

        if idx != 0 {
            ans += min(k as i64 - s[idx - 1] as i64, s[idx] as i64 - k as i64);
        }
    }

    println!("{}", ans);
}
