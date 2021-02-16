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
    let mut g = sc.get_as_vec::<usize>();
    let inf = 1_000_000_010;
    let mut d = vec![inf; g[0]];
    let mut edges = vec![Vec::new(); g[0]];

    for _ in 0..g[1] {
        sc.new_line();
        let mut a = sc.get_as_vec::<usize>();
        edges[a[0]].push(Edge {
            to: a[1],
            dist: a[2] as i64,
        });
    }
    d[g[2]] = 0;

    let mut heap: BinaryHeap<Reverse<Edge>> = BinaryHeap::new();
    heap.push(Reverse(Edge { to: g[2], dist: 0 }));

    while !&heap.is_empty() {
        let Edge { to, dist } = heap.pop().unwrap().0;
        let t = to as usize;
        if d[to] < dist {
            continue;
        }
        for n in &edges[t] {
            if d[n.to] > d[to] + n.dist {
                d[n.to] = d[to] + n.dist;
                heap.push(Reverse(Edge {
                    to: n.to,
                    dist: n.dist,
                }));
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
