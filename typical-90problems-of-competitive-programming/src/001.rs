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

    sc.new_line();
    let N: i64 = sc.get();
    let L: i64 = sc.get();
    sc.new_line();
    let k: i64 = sc.get();
    sc.new_line();
    let mut v: Vec<i64> = sc.get_as_vec();
    let mut arr: Vec<i64> = Vec::new();
    arr.push(v[0]);
    for i in 1..v.len() {
        arr.push(v[i] - v[i - 1]);
    }
    arr.push(L - v[v.len() - 1]);
    let mut l = 0;
    let mut r = L + 1;
    let mut m = 0;
    let mut ret = 0;
    while r - l > 1 {
        m = (r - l) / 2 + l;
        if isOk(m, &arr, k) {
            l = m;
        } else {
            r = m;
        }
    }
    println!("{}", l);
}

fn isOk(m: i64, arr: &Vec<i64>, k: i64) -> bool {
    let mut tmp = 0;
    let mut cnt = 1;
    for a in arr.iter() {
        tmp += a;
        if tmp >= m {
            cnt += 1;
            tmp = 0;
        }
    }
    cnt > k + 1
}
