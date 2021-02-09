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

    sc.new_line();
    let n = sc.get();

    let inf: i64 = 1_000_000_000_000;
    let mut matrixes = vec![];

    for i in 0..n {
        sc.new_line();
        let v = sc.get_as_vec::<usize>();
        matrixes.push(v);
    }

    let mut dp = vec![vec![inf; n]; n];
    for i in 0..n {
        dp[i][i] = 0;
    }

    for i in 1..n {
        for l in 0..n - i {
            for k in 0..i {
                dp[l][l + i] = min(
                    dp[l][l + i],
                    dp[l][l + k]
                        + (matrixes[l][0] * matrixes[l + k][1] * matrixes[l + i][1]) as i64
                        + dp[l + k + 1][l + i],
                );
            }
        }
    }

    println!("{}", dp[0][n - 1]);
}
