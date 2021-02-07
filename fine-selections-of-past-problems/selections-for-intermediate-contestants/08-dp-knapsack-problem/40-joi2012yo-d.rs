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
    let n = sc.get::<usize>();
    let k = sc.get::<usize>();

    let mut planned = vec![4; n];

    for i in 0..k {
        sc.new_line();
        let a = sc.get_as_vec::<usize>();
        planned[a[0] - 1] = a[1] - 1;
    }

    let mut dp = vec![vec![vec![0; 3]; 4]; n - 1];

    let md = 10_000;

    // Initialization
    if planned[0] != 4 && planned[1] != 4 {
        dp[0][planned[0]][planned[1]] = 1;
    } else if planned[0] != 4 {
        dp[0][planned[0]][0] = 1;
        dp[0][planned[0]][1] = 1;
        dp[0][planned[0]][2] = 1;
    } else if planned[1] != 4 {
        dp[0][0][planned[1]] = 1;
        dp[0][1][planned[1]] = 1;
        dp[0][2][planned[1]] = 1;
    } else {
        for i in 0..3 {
            for j in 0..3 {
                dp[0][i][j] = 1;
            }
        }
    }

    for k in 0..3 {
        dp[0][3][0] += dp[0][k][0];
        dp[0][3][1] += dp[0][k][1];
        dp[0][3][2] += dp[0][k][2];
    }

    // dp
    for i in 1..n - 1 {
        if planned[i + 1] != 4 {
            let kind = planned[i + 1];
            for j in 0..3 {
                dp[i][j][kind] += dp[i - 1][3][j];
                if j == kind {
                    dp[i][j][kind] -= dp[i - 1][j][kind];
                }
                dp[i][j][kind] = (dp[i][j][kind] + md) % md;
            }
        } else {
            for j in 0..3 {
                for k in 0..3 {
                    dp[i][j][k] += dp[i - 1][3][j];
                    dp[i][j][k] %= md;
                }
                dp[i][j][j] -= dp[i - 1][j][j];
                dp[i][j][j] = (dp[i][j][j] + md) % md;
            }
        }
        for k in 0..3 {
            dp[i][3][0] += dp[i][k][0];
            dp[i][3][1] += dp[i][k][1];
            dp[i][3][2] += dp[i][k][2];

            dp[i][3][0] %= md;
            dp[i][3][1] %= md;
            dp[i][3][2] %= md;
        }
    }

    println!(
        "{}",
        (dp[n - 2][3][0] + dp[n - 2][3][1] + dp[n - 2][3][2]) % md
    );
}
