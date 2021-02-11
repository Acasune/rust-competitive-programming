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

    let mut cakes: Vec<usize> = vec![0; n];

    for i in 0..n {
        sc.new_line();
        cakes[i] = sc.get();
    }

    let mut dp = vec![vec![0; n]; n];
    if n % 2 == 1 {
        for i in 0..n {
            dp[i][i] = cakes[i];
        }
    }

    for j in 1..n {
        for i in 0..n {
            dp[i][(i + j) % n] = {
                if (i + i + j) % 2 != n % 2 {
                    //bro
                    max(
                        dp[(i + 1) % n][(i + j) % n] + cakes[i],
                        dp[i][(i + j + n - 1) % n] + cakes[(i + j) % n],
                    )
                } else {
                    //sis
                    if cakes[i] > cakes[(i + j) % n] {
                        dp[(i + 1) % n][(i + j) % n]
                    } else {
                        dp[i][(i + j + n - 1) % n]
                    }
                }
            };
        }
    }

    let ans = {
        let mut tmp = 0;
        for i in 0..n {
            tmp = max(tmp, dp[i][(n + i - 1) % n]);
        }
        tmp
    };

    println!("{}", ans);
}
