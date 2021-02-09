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

    let upper = 1_000_000;

    let mut tri = vec![1];
    for i in 1.. {
        if i + 1 + tri[i - 1] > upper {
            break;
        }
        tri.push(i + 1 + tri[i - 1]);
    }

    for i in 1..tri.len() {
        tri[i] += tri[i - 1];
    }

    let mut tetra = vec![0];
    let mut tetra_odd = vec![0];
    for i in 0..tri.len() {
        if tri[i] > upper {
            break;
        }
        tetra.push(tri[i]);
        if tri[i] % 2 == 1 {
            tetra_odd.push(tri[i]);
        }
    }

    let mut dp = vec![vec![upper; 2 * upper]; 2];

    for &e in &tetra {
        dp[0][e] = 1;
    }
    for &e in &tetra_odd {
        dp[1][e] = 1;
    }

    for i in 1..upper {
        for &e in &tetra {
            if i + e > upper {
                continue;
            }
            dp[0][i + e] = min(dp[0][i] + 1, dp[0][i + e]);
        }
        for &e in &tetra_odd {
            if i + e > upper {
                continue;
            }
            dp[1][i + e] = min(dp[1][i] + 1, dp[1][i + e]);
        }
    }
    loop {
        sc.new_line();
        let n = sc.get::<usize>();
        if n == 0 {
            break;
        }
        println!("{} {}", dp[0][n], dp[1][n]);
    }
}
