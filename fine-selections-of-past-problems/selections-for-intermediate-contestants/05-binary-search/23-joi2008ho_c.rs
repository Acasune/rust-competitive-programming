// https://atcoder.jp/contests/joi2009ho/tasks/joi2009ho_b

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
    let n = sc.get::<usize>();
    let m = sc.get::<i64>();
    let mut a = vec![0];

    for i in 0..n {
        sc.new_line();
        let tmp = sc.get::<i64>();
        a.push(tmp);
    }
    let mut scores = vec![];
    for i in 0..n + 1 {
        for j in i..n + 1 {
            scores.push(a[i] + a[j]);
        }
    }
    scores.sort();
    let scores_cp = scores.clone();

    let mut ans = 0;

    for s in scores {
        let rem = m - s;
        if rem < 0 {
            continue;
        }
        if rem == 0 {
            ans = m;
            break;
        }
        let idx = scores_cp
            .binary_search_by_key(&(2 * (rem + 1)), |a| 2 * a + 1)
            .err()
            .unwrap();
        ans = max(ans, s + scores_cp[idx - 1]);
    }

    println!("{}", ans);
}
