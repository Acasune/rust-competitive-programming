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
    let mut n = sc.get::<usize>();
    let mut k = sc.get::<usize>();

    let inf = 1_000_000_010;
    let mut edges = vec![Vec::new(); n];

    for _ in 0..k {
        sc.new_line();
        let a = sc.get::<usize>();
        if a == 1 {
            let b = sc.get::<usize>() - 1;
            let c = sc.get::<usize>() - 1;
            let d = sc.get::<i64>();
            edges[b].push(Edge { to: c, dist: d });
            edges[c].push(Edge { to: b, dist: d });
        } else {
            let from = sc.get::<usize>() - 1;
            let to = sc.get::<usize>() - 1;
            let mut d = vec![inf; n];
            d[from] = 0;

            let mut heap: BinaryHeap<Reverse<Edge>> = BinaryHeap::new();
            heap.push(Reverse(Edge { to: from, dist: 0 }));

            while !&heap.is_empty() {
                let Edge {
                    to: next_to,
                    dist: next_dist,
                } = heap.pop().unwrap().0;
                if d[next_to as usize] < next_dist {
                    continue;
                }
                for n in &edges[next_to as usize] {
                    if d[n.to] > d[next_to as usize] + n.dist {
                        d[n.to] = d[next_to as usize] + n.dist;
                        heap.push(Reverse(Edge {
                            to: n.to,
                            dist: n.dist,
                        }));
                    }
                }
            }
            println!("{}", if d[to] == inf { -1 } else { d[to] });
        }
    }
}
