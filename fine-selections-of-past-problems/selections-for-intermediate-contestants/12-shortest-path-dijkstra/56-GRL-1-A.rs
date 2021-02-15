use std::{
    cmp::{max, min, Reverse},
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
    let mut g = sc.get_as_vec::<usize>();
    let inf = 1_000_000_010;
    let mut d = vec![inf; g[0]];
    let mut costs = vec![Vec::new(); g[0]];

    for _ in 0..g[1] {
        sc.new_line();
        let mut a = sc.get_as_vec::<usize>();
        costs[a[0]].push(vec![a[2], a[1]]);
    }
    d[g[2]] = 0;

    let mut heap: BinaryHeap<Reverse<Vec<usize>>> = BinaryHeap::new();
    heap.push(Reverse(vec![0, g[2]]));

    while &heap.len() > &0 {
        let mut c = heap.pop().unwrap().0;
        let t = c[1];
        if (d[t] < c[0]) {
            continue;
        }
        for n in &costs[t] {
            if d[n[1]] > d[t] + n[0] {
                d[n[1]] = d[t] + n[0];
                heap.push(Reverse(vec![d[n[1]], n[1]]));
            }
        }
    }

    for i in 0..g[0] {
        if d[i] == inf {
            println!("INF");
        } else {
            println!("{}", d[i]);
        }
    }
}
