use itertools::Itertools;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{i64, io::*, str::FromStr, usize};

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
    let H = sc.get::<usize>();
    let W = sc.get::<usize>();

    let mut maze1 = vec![Vec::<i64>::new(); H];
    let mut maze2 = vec![Vec::<i64>::new(); H];
    for h in 0..H {
        sc.new_line();
        maze1[h] = sc.get_as_vec::<i64>();
    }
    for h in 0..H {
        sc.new_line();
        maze2[h] = sc.get_as_vec::<i64>();
    }
    let mut ret = 0;
    for h in 0..H - 1 {
        for w in 0..W - 1 {
            let diff = maze1[h][w] - maze2[h][w];
            ret += diff.abs();
            maze2[h][w] += diff;
            maze2[h][w + 1] += diff;
            maze2[h + 1][w] += diff;
            maze2[h + 1][w + 1] += diff;
        }
    }
    for j in 0..H {
        if maze1[j][W - 1] != maze2[j][W - 1] {
            println!("No");
            return;
        }
    }
    for j in 0..W {
        if maze1[H - 1][j] != maze2[H - 1][j] {
            println!("No");
            return;
        }
    }
    println!("Yes");
    println!("{}", ret);
}
