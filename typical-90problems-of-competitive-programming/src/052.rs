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

fn dfs(dp: &mut Vec<Vec<usize>>, size: usize, sum: usize, idx: usize, al: &Vec<usize>, N: usize) {
    if idx >= N {
        return;
    }
    //use
    dp[size + 1].push(sum + al[idx]);
    dfs(dp, size + 1, sum + al[idx], idx + 1, al, N);
    //not use
    dfs(dp, size, sum, idx + 1, al, N);
}

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let N: usize = sc.get();
    let mut dices = vec![Vec::<usize>::new(); N];
    let md = 1_000_000_007;
    for i in 0..N {
        sc.new_line();
        dices[i] = sc.get_as_vec::<usize>();
    }
    let mut dp = vec![vec![0; 6]; N];
    for i in 0..6 {
        dp[0][i] = dices[0][i];
    }
    for i in 1..N {
        for j in 0..6 {
            for k in 0..6 {
                dp[i][j] += dp[i - 1][k] * dices[i][j];
                dp[i][j] %= md;
            }
        }
    }
    let mut ans = 0;
    for i in 0..6 {
        ans += dp[N - 1][i];
        ans %= md;
    }
    println!("{}", ans);
}
