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
    input! {
        H:usize,W:usize,x:usize,y:usize,
        S:[String;H],
    }
    let mut maze = Vec::<Vec<char>>::new();
    for i in 0..H {
        maze.push(S[i].chars().collect::<Vec<char>>());
    }
    let Y = x - 1;
    let X = y - 1;
    let mut ans = 0;
    for i in (0..X).rev() {
        if maze[Y][i] == '#' {
            break;
        }
        ans += 1;
    }
    for i in (X..W) {
        if maze[Y][i] == '#' {
            break;
        }
        ans += 1;
    }
    for i in (0..Y).rev() {
        if maze[i][X] == '#' {
            break;
        }
        ans += 1;
    }
    for i in (Y..H) {
        if maze[i][X] == '#' {
            break;
        }
        ans += 1;
    }

    println!("{}", ans - 1);
}
