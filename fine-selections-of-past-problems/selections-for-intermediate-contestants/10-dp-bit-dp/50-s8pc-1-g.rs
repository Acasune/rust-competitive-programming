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

    let inf = 1 << 60;

    sc.new_line();
    let v = sc.get::<usize>();
    let e = sc.get::<usize>();

    let mut edges = vec![vec![vec![inf, inf]; v]; v];

    for i in 0..e {
        sc.new_line();
        let v = sc.get_as_vec::<usize>();
        edges[v[0] - 1][v[1] - 1][0] = v[2]; //dist
        edges[v[1] - 1][v[0] - 1][0] = v[2]; //dist
        edges[v[0] - 1][v[1] - 1][1] = v[3]; //time
        edges[v[1] - 1][v[0] - 1][1] = v[3]; //time
    }

    let mut dp = vec![vec![vec![inf, 0]; v]; (1 << v)];
    dp[0][0][0] = 0; // shortest dist
    dp[0][0][1] = 1; // number of paths

    for s in 1..(1 << v) {
        // to
        for i in 0..v {
            if s & (1 << i) != 0 {
                // from
                if s & (1 << i) != 0 {
                    for j in 0..v {
                        if dp[s - (1 << i)][j][0] + edges[j][i][0] > edges[j][i][1] {
                            continue;
                        }
                        if dp[s][i][0] < dp[s - (1 << i)][j][0] + edges[j][i][0] {
                            continue;
                        } else if dp[s][i][0] == dp[s - (1 << i)][j][0] + edges[j][i][0] {
                            dp[s][i][1] += dp[s - (1 << i)][j][1];
                        } else {
                            dp[s][i][0] = dp[s - (1 << i)][j][0] + edges[j][i][0];
                            dp[s][i][1] = dp[s - (1 << i)][j][1];
                        }
                    }
                }
            }
        }

        println!(
            "{}",
            if dp[(1 << v) - 1][0][1] == 0 {
                "IMPOSSIBLE".to_string()
            } else {
                format!("{} {}", dp[(1 << v) - 1][0][0], dp[(1 << v) - 1][0][1])
            }
        );
    }
}
