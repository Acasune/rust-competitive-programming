// https://atcoder.jp/contests/joi2009ho/tasks/joi2009ho_b

use std::{
    cmp::{max, min},
    io::*,
    str::FromStr,
    usize,
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
    let n = sc.get::<i64>();
    sc.new_line();
    let mut a = sc.get_as_vec::<i64>();
    sc.new_line();
    let mut b = sc.get_as_vec::<i64>();
    sc.new_line();
    let mut c = sc.get_as_vec::<i64>();

    a.sort();

    b.sort();

    c.sort();

    let mut ans: i64 = 0;

    for b in b {
        let mut ai = a
            .binary_search_by_key(&(2 * b), |a| 2 * a + 1)
            .err()
            .unwrap();
        let mut ci = c
            .binary_search_by_key(&(2 * (b + 1)), |a| 2 * a + 1)
            .err()
            .unwrap();

        if ci == n as usize {
            continue;
        }
        // if b == c[ci] {
        //     ci = min(ci + 1, n as usize);
        // }
        ans += ai as i64 * (n - ci as i64).max(0);
    }

    println!("{}", ans);
}
