use std::{
    cmp::{max, min, Ordering, Reverse},
    collections::{BinaryHeap, HashSet, VecDeque},
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
    from: usize,
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

    let mut taxis = vec![vec![0, 0]; n];
    for i in 0..n {
        sc.new_line();
        let a = sc.get::<usize>();
        let b = sc.get::<usize>();
        taxis[i][0] = a;
        taxis[i][1] = b;
    }

    let mut roads = vec![Vec::new(); n];
    for i in 0..k {
        sc.new_line();
        let a = sc.get::<usize>() - 1;
        let b = sc.get::<usize>() - 1;
        roads[a].push(b);
        roads[b].push(a);
    }

    let inf = 1_000_000_000_010;

    // dijkstra

    let mut costs = vec![vec![inf; n]; n];

    for i in 0..n {
        let mut que = VecDeque::new();
        let mut dist = vec![inf; n];
        que.push_back(i);
        dist[i] = 0;
        while !&que.is_empty() {
            let from = que.pop_front().unwrap();
            if dist[from] >= taxis[i][1] {
                continue;
            }
            for &e in &roads[from] {
                if dist[e] != inf {
                    continue;
                }
                dist[e] = dist[from] + 1;

                costs[i][e] = taxis[i][0];

                que.push_back(e);
            }
        }
    }

    let mut costs_from_0 = vec![inf; n];
    costs_from_0[0] = 0;

    let mut heap = BinaryHeap::new();

    heap.push(Reverse(Edge { from: 0, dist: 0 }));

    while !&heap.is_empty() {
        let Edge { from: f, dist: d } = heap.pop().unwrap().0;
        if costs_from_0[f as usize] < d as usize {
            continue;
        }
        for i in 0..n {
            if costs[f][i] == inf || i == f {
                continue;
            }
            if costs_from_0[i] > costs_from_0[f] + costs[f][i] {
                costs_from_0[i] = costs_from_0[f] + costs[f][i];
                heap.push(Reverse(Edge {
                    from: i,
                    dist: costs[f][i] as i64,
                }));
            }
        }
    }
    println!("{}", costs_from_0[n - 1])
}
