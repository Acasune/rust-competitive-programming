use itertools::Itertools;
use std::cmp::{max, min, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Binary;
use std::{i64, io::*, str::FromStr};

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

fn dfs(dp: &mut Vec<i64>, pos: usize, pre: usize, G: &Vec<Vec<usize>>) {
    dp[pos] = 1;
    for e in &G[pos] {
        if *e == pre {
            continue;
        }
        dfs(dp, *e, pos, G);
        dp[pos] += dp[*e];
    }
}

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let H: usize = sc.get();
    let W: usize = sc.get();
    sc.new_line();
    let rs = sc.get::<i64>() - 1;
    let cs = sc.get::<i64>() - 1;
    sc.new_line();
    let rt = sc.get::<i64>() - 1;
    let ct = sc.get::<i64>() - 1;
    let mut maze = vec![Vec::<char>::new(); H];
    let mut visited = vec![vec![vec![1_000_000_000; 4]; W]; H];
    let DYX = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for h in 0..H {
        sc.new_line();
        let a = sc.get::<String>().chars().collect::<Vec<char>>();
        maze[h] = a;
    }
    let mut deq = VecDeque::<(i64, i64, i64)>::new();

    for i in 0..4 {
        visited[rs as usize][cs as usize][i] = 0;
        deq.push_back((rs, cs, i as i64));
    }

    while !deq.is_empty() {
        let (y, x, dir) = deq.pop_front().unwrap();

        for i in 0..4 {
            let ny = y + DYX[i].0;
            let nx = x + DYX[i].1;
            let cost = if dir == i as i64 {
                visited[y as usize][x as usize][dir as usize]
            } else {
                visited[y as usize][x as usize][dir as usize] + 1
            };

            if ny < 0
                || (H as i64) <= ny
                || nx < 0
                || (W as i64) <= nx
                || maze[ny as usize][nx as usize] == '#'
                || visited[ny as usize][nx as usize][i as usize] <= cost
            {
                continue;
            }

            visited[ny as usize][nx as usize][i as usize] = cost;
            if dir == i as i64 {
                deq.push_front((ny, nx, i as i64));
            } else {
                deq.push_back((ny, nx, i as i64));
            }
        }
    }
    println!(
        "{}",
        visited[rt as usize][ct as usize].iter().min().unwrap()
    );
}
