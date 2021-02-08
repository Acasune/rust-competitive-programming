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

    let mut flag = vec![];

    for i in 0..5 {
        sc.new_line();
        let tmp: Vec<char> = sc.get::<String>().chars().collect();
        flag.push(tmp);
    }

    let mut colors = vec![vec![0; n]; 3];

    for j in 0..n {
        for i in 0..5 {
            if flag[i][j] == 'R' {
                colors[0][j] += 1
            }
            if flag[i][j] == 'B' {
                colors[1][j] += 1
            }
            if flag[i][j] == 'W' {
                colors[2][j] += 1
            }
        }
    }

    // count squares whose columns' color is not changed;

    let mut dp = vec![vec![0; n]; 3];

    dp[0][0] = colors[0][0];
    dp[1][0] = colors[1][0];
    dp[2][0] = colors[2][0];

    for j in 1..n {
        for i in 0..3 {
            let left_color = {
                let mut tmp = 0;
                for k in 0..3 {
                    if i == k {
                        continue;
                    };
                    tmp = max(tmp, dp[k][j - 1]);
                }
                tmp
            };
            dp[i][j] += left_color + colors[i][j];
        }
    }
    let cnt = {
        let mut tmp = 0;
        for i in 0..3 {
            tmp = max(tmp, dp[i][n - 1]);
        }
        tmp
    };

    println!("{}", 5 * n - cnt);
}
