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

#[derive(Copy, Clone, Eq, PartialEq)]
struct Edge {
    to: usize,
    dist: i64,
}

impl Ord for Edge {
    fn cmp(&self, other: &Edge) -> Ordering {
        if self.dist > other.dist {
            Ordering::Greater
        } else if self.dist < other.dist {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}
impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Edge) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let mut v = sc.get::<usize>();
    let mut e = sc.get::<usize>();
    let inf = 1_000_000__000_000_010;
    let mut dists = vec![vec![inf; v]; v];
    for i in 0..e {
        sc.new_line();
        let s = sc.get::<usize>();
        let t = sc.get::<usize>();
        let d = sc.get::<i64>();
        dists[s][t] = d;
    }
    for i in 0..v {
        dists[i][i] = 0;
    }

    for k in 0..v {
        for i in 0..v {
            for j in 0..v {
                dists[i][j] = min(dists[i][j], dists[i][k] + dists[k][j]);
            }
        }
    }

    for i in 0..v {
        if dists[i][i] < 0 {
            println!("NEGATIVE CYCLE");
            return;
        };
    }

    for v in &dists {
        for i in 0..v.len() {
            if i == 0 {
                print!(
                    "{}",
                    if v[i] > inf / 2 {
                        "INF".to_string()
                    } else {
                        v[i].to_string()
                    }
                );
            } else {
                print!(
                    " {}",
                    if v[i] > inf / 2 {
                        "INF".to_string()
                    } else {
                        v[i].to_string()
                    }
                );
            }
        }
        println!("");
    }
}
