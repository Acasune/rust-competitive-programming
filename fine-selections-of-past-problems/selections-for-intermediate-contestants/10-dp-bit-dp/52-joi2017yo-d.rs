use std::{cmp::min, io::*, str::FromStr};

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

    let inf: i64 = 1 << 60;

    sc.new_line();
    let n = sc.get::<usize>();
    let m = sc.get::<usize>();

    let mut arr = vec![0; n];
    let mut totals = vec![0; m];
    for i in 0..n {
        sc.new_line();
        let tmp = sc.get::<usize>() - 1;
        arr[i] = tmp;
        totals[tmp] += 1;
    }

    let mut cs = vec![vec![0; m]; n + 1];

    for i in 1..n + 1 {
        for j in 0..m {
            if arr[i - 1] == j {
                cs[i][j] = cs[i - 1][j] + 1;
            } else {
                cs[i][j] = cs[i - 1][j];
            }
        }
    }

    let mut dp = vec![inf; (1 << m)];
    dp[0] = 0;

    for s in 0..(1 << m) {
        // using the i th doll
        for i in 0..m {
            if s & (1 << i) > 0 {
                continue;
            }
            let mut total = 0;
            for j in 0..m {
                if s & (1 << j) > 0 {
                    total += totals[j];
                }
            }

            dp[s + (1 << i)] = min(
                dp[s + (1 << i)],
                dp[s]
                    + (totals[i] - cs[total as usize + totals[i] as usize][i]
                        + cs[total as usize][i]),
            );
        }
    }

    println!("{}", dp[(1 << m) - 1]);
}
