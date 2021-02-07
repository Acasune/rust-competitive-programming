use std::{
    cmp::{max, min},
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
    let m = sc.get::<usize>();
    let n = sc.get::<usize>();

    let mut coins = vec![];

    sc.new_line();
    let a = sc.get_as_vec::<usize>();
    for &t in a.iter() {
        coins.push(t);
    }

    let inf = 10_010;

    let mut dp = vec![vec![inf; m + 1]; n + 1];
    dp[0][0] = 0;

    for i in 0..n {
        for w in 0..m + 1 {
            if (w as i64 - coins[i] as i64) >= 0 {
                dp[i + 1][w] = min(min(dp[i][w], dp[i + 1][w - coins[i]] + 1), dp[i + 1][w]);
            } else {
                dp[i + 1][w] = min(dp[i + 1][w], dp[i][w])
            }
        }
    }
    println!("{}", dp[n][m]);
}
