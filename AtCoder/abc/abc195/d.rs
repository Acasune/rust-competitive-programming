use proconio::input;
use std::iter::FromIterator;
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
    input! {
        N:usize, M:usize,Q:usize,
        V: [(usize,usize);N],
        X:[usize;M],
        QS:[(usize,usize);Q],
    }
    let mut items: Vec<(usize, usize)> = V.clone();
    items.sort_by(|&(w1, v1), &(w2, v2)| v2.cmp(&v1));
    for q in 0..Q {
        let mut ans = 0;
        let l = QS[q].0 - 1;
        let r = QS[q].1 - 1;
        let mut boxes = X[..l]
            .iter()
            .chain(X[r + 1..].iter())
            .cloned()
            .collect::<Vec<_>>();
        boxes.sort();
        let mut used = vec![false; N];
        for &b in &boxes {
            for i in 0..N {
                if items[i].0 <= b && !used[i] {
                    ans += items[i].1;
                    used[i] = true;
                    break;
                }
            }
        }
        println!("{}", ans);
    }
}
