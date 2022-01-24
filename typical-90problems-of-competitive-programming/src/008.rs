use std::cmp::{max, min};
use std::collections::VecDeque;
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
    let N: usize = sc.get();
    sc.new_line();
    let cs: Vec<char> = sc.get::<String>().chars().collect();
    let mut dp = vec![vec![0; 8]; N + 1];
    let c = cs[0];
    let MOD = 1_000_000_007;
    dp[0][0] = 1;
    for i in 0..N {
        for j in 0..8 {
            let c = cs[i];
            dp[i + 1][j] += dp[i][j];
            if c == 'a' && j == 0 {
                dp[i + 1][j + 1] = dp[i][j];
            } else if c == 't' && j == 1 {
                dp[i + 1][j + 1] = dp[i][j];
            } else if c == 'c' && j == 2 {
                dp[i + 1][j + 1] = dp[i][j];
            } else if c == 'o' && j == 3 {
                dp[i + 1][j + 1] = dp[i][j];
            } else if c == 'd' && j == 4 {
                dp[i + 1][j + 1] = dp[i][j];
            } else if c == 'e' && j == 5 {
                dp[i + 1][j + 1] = dp[i][j];
            } else if c == 'r' && j == 6 {
                dp[i + 1][j + 1] = dp[i][j];
            }
        }
        for j in 0..8 {
            dp[i + 1][j] %= MOD;
        }
    }
    println!("{}", dp[cs.len()][7]);
}
