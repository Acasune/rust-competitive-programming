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
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    loop {
        sc.new_line();
        let n = sc.get();
        if n == 0 {
            break;
        }
        sc.new_line();
        let daruma = sc.get_as_vec::<i64>();
        let mut dp = vec![vec![0 as i64; n]; n];

        for i in 0..n - 1 {
            if i64::abs(daruma[i] as i64 - daruma[i + 1] as i64) <= 1 {
                dp[i][i + 1] = 2;
            }
        }

        for i in 2..n {
            for l in 0..n - i {
                if dp[l + 1][l + i - 1] == i as i64 - 1 {
                    dp[l][i + l] = {
                        if i64::abs(daruma[l] as i64 - daruma[i + l] as i64) <= 1 {
                            max(dp[l][i + l], i as i64 + 1)
                        } else {
                            max(dp[l][i + l], i as i64 - 1)
                        }
                    }
                }
                for k in l..(i + l) {
                    dp[l][i + l] = max(dp[l][i + l], dp[l][k] + dp[k + 1][i + l])
                }
            }
        }
        println!("{}", dp[0][n - 1]);
    }
}
