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

    let inf = 256 * 200_000;
    let y0 = 128;

    loop {
        sc.new_line();
        let n = sc.get::<usize>();
        let m = sc.get::<usize>();
        if n == 0 && m == 0 {
            break;
        }

        let mut code_book = vec![0; m];

        for i in 0..m {
            sc.new_line();
            code_book[i] = sc.get::<i64>();
        }
        let mut x_v = vec![0; n];
        for i in 0..n {
            sc.new_line();
            x_v[i] = sc.get::<i64>();
        }
        let mut dp = vec![vec![inf; 256]; n + 1];
        dp[0][y0] = 0;

        // 配るdpでいく

        for i in 0..n {
            for j in 0..256 {
                for &c in &code_book {
                    let y_1 = {
                        if j + c < 0 {
                            0
                        } else if j + c > 255 {
                            255
                        } else {
                            j + c
                        }
                    };
                    let squared_sum = (y_1 - x_v[i]) * (y_1 - x_v[i]);
                    dp[i + 1][y_1 as usize] =
                        min(dp[i + 1][y_1 as usize], dp[i][j as usize] + squared_sum);
                }
            }
        }
        let ans = {
            let mut tmp = inf;
            for i in 0..256 {
                tmp = min(tmp, dp[n][i]);
            }
            tmp
        };
        println!("{}", ans);
    }
}
