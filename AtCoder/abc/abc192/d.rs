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
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let mut X = sc.get::<String>();
    sc.new_line();
    let mut M = sc.get::<i128>();
    let mut base: i128 = X
        .chars()
        .max_by_key(|c| c.to_digit(10).unwrap())
        .unwrap()
        .to_digit(10)
        .unwrap() as i128
        + 1;
    let X: Vec<char> = X.chars().collect();
    if X.len() == 1 {
        println!("{}", if base - 1 <= M { 1 } else { 0 });
        return;
    }
    let mut l = 0;
    let mut r = M + 1;

    while (r - l > 1) {
        let m = (r - l) / 2 + l;
        let flg = on_base_to_val(m as i128, &X, M);
        // println!("{} {}", flg, m);
        if flg {
            l = m;
        } else {
            r = m;
        }
    }

    println!("{}", 0.max(r - base));
}

fn on_base_to_val(base: i128, t: &Vec<char>, M: i128) -> bool {
    let mut nn = 0;
    let mut oc = 1;
    for i in (0..t.len()).rev() {
        nn += t[i].to_digit(10).unwrap() as i128 * oc;
        if (M + 1) / base < oc {
            if i == 0 {
                break;
            } else {
                return false;
            }
        }
        oc *= base;
    }
    return nn <= M;
}
