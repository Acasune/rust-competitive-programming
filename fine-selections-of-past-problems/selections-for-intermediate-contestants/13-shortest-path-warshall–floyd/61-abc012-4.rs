use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{BinaryHeap, HashSet},
    fmt::Binary,
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
    let mut n = sc.get::<usize>();
    let mut m = sc.get::<usize>();
    let inf = 1_000_000__000_000_010;
    let mut dists = vec![vec![inf; n]; n];
    for i in 0..m {
        sc.new_line();
        let s = sc.get::<usize>() - 1;
        let t = sc.get::<usize>() - 1;
        let d = sc.get::<i64>();
        dists[s][t] = d;
        dists[t][s] = d;
    }
    for i in 0..n {
        dists[i][i] = 0;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dists[i][j] = min(dists[i][j], dists[i][k] + dists[k][j]);
            }
        }
    }

    let ans = {
        let mut ret = inf;

        for i in 0..n {
            let mut tmp = -1;
            for j in 0..n - 1 {
                if j < i {
                    tmp = max(tmp, dists[j][i]);
                } else {
                    tmp = max(tmp, dists[i][j + 1]);
                }
            }
            ret = min(ret, tmp);
        }
        ret
    };

    println!("{}", ans);
}
