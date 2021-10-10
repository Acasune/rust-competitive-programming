#[allow(unused_imports)]
use std::cmp::{max, min};
use std::{
    cell::RefCell,
    collections::HashMap,
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

#[allow(unused_variables)]
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n = sc.get::<usize>();
    sc.new_line();
    let p = sc.get_as_vec::<f64>();
    sc.new_line();
    let q = sc.get_as_vec::<f64>();

    let mut x = vec![0_f64; p.len() + q.len()];
    let mut memo = vec![vec![0_f64; 2 * n + 2]; 2 * n + 2];
    for i in 0..x.len() {
        if i % 2 == 0 {
            x[i] = q[i / 2];
        } else {
            x[i] = p[i / 2];
        }
    }
    for i in 0..x.len() {
        memo[i][i + 1] = x[i];
    }
    for j in 2..2 * n + 2 {
        //  col
        for l in 0..2 * n + 2 - j {
            // row
            let r = l + j;
            // println!("{} {}", l, r);
            let mut min = None;
            let mut base = 0_f64;
            for mid in l..r {
                base += x[mid];
                if mid % 2 == 0 {
                    continue;
                }
                let a = memo[l][mid] + memo[mid + 1][r];
                if min.is_none() {
                    min = Some(a);
                } else if let Some(b) = min {
                    if b > a {
                        min = Some(a);
                    }
                }
            }
            memo[l][r] = base + min.unwrap();
        }
    }
    println!("{}", memo[0][x.len()]);
}
