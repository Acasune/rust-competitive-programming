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

    let inf = 1_000_000_100;

    sc.new_line();
    let v = sc.get::<usize>();
    let e = sc.get::<usize>();

    let mut edges = vec![vec![inf; v]; v];

    for i in 0..e {
        sc.new_line();
        let v = sc.get_as_vec::<usize>();
        edges[v[0]][v[1]] = v[2];
    }

    let mut dp = vec![vec![inf; v]; (1 << v)];
    dp[0][0] = 0;

    for s in 1..(1 << v) {
        // to
        for i in 0..v {
            // from
            if s & (1 << i) != 0 {
                for j in 0..v {
                    // println!("{} {}", s, s | (1 << i));
                    dp[s][i] = min(dp[s][i], dp[s - (1 << i)][j] + edges[j][i]);
                }
            }
        }
    }

    println!(
        "{}",
        if dp[(1 << v) - 1][0] == inf {
            -1
        } else {
            dp[(1 << v) - 1][0] as i64
        }
    );
}
