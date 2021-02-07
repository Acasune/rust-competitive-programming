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
    let d = sc.get::<usize>();
    let n = sc.get::<usize>();

    let mut degrees = vec![0; d];

    for i in 0..d {
        sc.new_line();
        degrees[i] = sc.get::<i64>();
    }

    let mut clothes = vec![];

    for i in 0..n {
        sc.new_line();
        let a = sc.get_as_vec::<i64>();
        clothes.push((a[0], a[1], a[2]));
    }

    let mut dp = vec![vec![vec![0 as i64; n]; n + 1]; d - 1];

    // Initialization
    for i in 0..n {
        if degrees[0] < clothes[i].0 || clothes[i].1 < degrees[0] {
            continue;
        }
        for j in 0..n {
            if degrees[1] < clothes[j].0 || clothes[j].1 < degrees[1] {
                continue;
            }
            dp[0][i][j] = i64::abs(clothes[i].2 - clothes[j].2);
        }
    }
    for i in 0..n {
        for j in 0..n {
            dp[0][n][j] = i64::max(dp[0][n][j], dp[0][i][j]);
        }
    }

    // dp
    for i in 1..d - 1 {
        for j in 0..n {
            if degrees[i] < clothes[j].0 || clothes[j].1 < degrees[i] {
                continue;
            }
            for k in 0..n {
                if degrees[i + 1] < clothes[k].0 || clothes[k].1 < degrees[i + 1] {
                    continue;
                }
                dp[i][j][k] = dp[i - 1][n][j] + i64::abs(clothes[j].2 - clothes[k].2);
            }
        }
        for j in 0..n {
            for k in 0..n {
                dp[i][n][k] = i64::max(dp[i][n][k], dp[i][j][k]);
            }
        }
    }
    let ans = {
        let mut tmp = 0;
        for i in 0..n {
            tmp = max(tmp, dp[d - 2][n][i])
        }
        tmp
    };

    println!("{}", ans);
}
