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
    let mut H: usize = sc.get();
    let mut W: usize = sc.get();
    let mut grid = vec![Vec::<usize>::new(); H];
    for i in 0..H {
        sc.new_line();
        grid[i] = sc.get_as_vec();
    }

    let mut ret = 0;
    for js in 0..(1 << H) {
        let mut memo = vec![0; H * W * 10];
        let mut idx_set = HashSet::<usize>::new();
        for i in 0..W {
            let (idx, sum) = is_same(js, i, &grid);
            if sum == 0 {
                continue;
            }
            memo[idx] += sum;
            idx_set.insert(idx);
        }
        let mut part_max = 0;
        for idx in &idx_set {
            part_max = max(part_max, memo[*idx]);
        }
        ret = max(part_max, ret);
    }
    println!("{}", ret);
}

fn is_same(js: usize, i: usize, grid: &Vec<Vec<usize>>) -> (usize, usize) {
    let mut val = 1_000_000_000;
    let mut cnt = 0;
    for j in 0..grid.len() {
        if js >> j & 1 != 1 {
            continue;
        }
        cnt += 1;
        if val == 1_000_000_000 {
            val = grid[j][i];
        } else if val != grid[j][i] {
            return (0, 0);
        }
    }
    return (val, cnt);
}
