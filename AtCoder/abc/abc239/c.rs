use std::{
    cmp::{max, min},
    collections::HashSet,
    io::*,
    str::FromStr,
};
use std::{collections::hash_set, iter::FromIterator};

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
    let x1 = sc.get::<i64>();
    let y1 = sc.get::<i64>();
    let x2 = sc.get::<i64>();
    let y2 = sc.get::<i64>();
    let DXY = [
        [-2, 1],
        [-1, 2],
        [1, 2],
        [2, 1],
        [2, -1],
        [1, -2],
        [-1, -2],
        [-2, -1],
    ];
    let mut points = Vec::<[i64; 2]>::new();
    for &dxy in &DXY {
        points.push([x1 + dxy[0], y1 + dxy[1]]);
    }
    let mut flg = false;
    for &dxy in &DXY {
        let xx = x2 + dxy[0];
        let yy = y2 + dxy[1];
        for &p in &points {
            if p[0] == xx && p[1] == yy {
                flg = true;
            }
        }
    }
    if flg {
        println!("Yes");
    } else {
        println!("No");
    }
}
