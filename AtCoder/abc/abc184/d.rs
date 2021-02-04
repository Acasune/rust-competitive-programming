use std::{
    cmp::{max, min},
    io::*,
    str::{Chars, FromStr},
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
    let a = sc.get::<usize>();
    let b = sc.get::<usize>();
    let c = sc.get::<usize>();

    let mut dp = vec![vec![vec![0 as f64; 101]; 101]; 101];
    dp[a][b][c] = 1.;

    for i in a..100 {
        for j in b..100 {
            for k in c..100 {
                if i > 0 {
                    dp[i + 1][j][k] += (dp[i][j][k]) * i as f64 / (i as f64 + j as f64 + k as f64);
                }
                if j > 0 {
                    dp[i][j + 1][k] += dp[i][j][k] * j as f64 / (i as f64 + j as f64 + k as f64);
                }
                if k > 0 {
                    dp[i][j][k + 1] += dp[i][j][k] * k as f64 / (i as f64 + j as f64 + k as f64);
                }
            }
        }
    }

    let mut ans: f64 = 0.;
    let s: f64 = (a + b + c) as f64;
    //dp[100][100][100] / ((100.0 - a as f64) * (100.0 - b as f64) * (100.0 - c as f64));
    for i in 0..100 {
        for j in 0..100 {
            let k = 100. + i as f64 + j as f64 - s;
            ans += k * dp[100][i][j];
            ans += k * dp[i][100][j];
            ans += k * dp[i][j][100];
        }
    }

    println!("{:.09}", ans);
}
