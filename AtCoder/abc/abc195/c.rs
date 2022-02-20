use proconio::input;
use std::iter::FromIterator;
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
    input! {S:String}

    let mut N = S.parse::<i64>().unwrap();
    let len = S.len();
    // if len < 4 {
    //     println!("{}", 0);
    //     return;
    // }
    let mut memo = vec![0i64; 20];
    let mut base = 9;
    let mut base2 = 0;
    for i in 1..len {
        memo[i] = base;
        base2 += base;
        base = base * 10;
    }
    memo[len] = N - base2;

    let mut ans = 0;
    for i in 4..=len {
        ans += get_len(i as i64, memo[i] as i64);
    }
    println!("{}", ans);
}

fn get_len(keta: i64, num: i64) -> i64 {
    let mut ans = keta / 3 - 1;
    if keta % 3 != 0 {
        ans += 1;
    }
    return ans * num;
}
