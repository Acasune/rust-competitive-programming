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
        H:usize,W:usize, A:usize,B:usize
    }
    let mut ans = 0;
    let mut used = vec![vec![false; W]; H];
    dfs(&mut used, &mut ans, 0, 0, A, H, W);

    println!("{}", ans);
}

fn dfs(used: &mut Vec<Vec<bool>>, ans: &mut usize, j: i64, i: i64, A: usize, H: usize, W: usize) {
    let DXY = [(1, 0), (0, 1)];

    if j == H as i64 {
        if A == 0 {
            *ans += 1;
        }
        return;
    }
    if i == W as i64 {
        dfs(used, ans, j + 1, 0, A, H, W);
        return;
    }

    if used[j as usize][i as usize] {
        dfs(used, ans, j, i + 1, A, H, W);
        return;
    }

    for i1 in 0..2 {
        let dy1 = j as i64 + DXY[i1].0;
        let dx1 = i as i64 + DXY[i1].1;
        if dy1 < H as i64 && dx1 < W as i64 && !used[dy1 as usize][dx1 as usize] && A > 0 {
            used[j as usize][i as usize] = true;
            used[dy1 as usize][dx1 as usize] = true;
            dfs(used, ans, j, i + 1, A - 1, H, W);
            used[j as usize][i as usize] = false;
            used[dy1 as usize][dx1 as usize] = false;
        }
    }

    dfs(used, ans, j, i + 1, A, H, W);
}
