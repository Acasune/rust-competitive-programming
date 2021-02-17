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
    let mut m = sc.get::<usize>();
    let mut k = sc.get::<usize>();
    let mut s = sc.get::<usize>();

    sc.new_line();
    let mut p = sc.get::<usize>();
    let mut q = sc.get::<usize>();

    let inf = 1_000_000_000_010;
    let mut edges = vec![Vec::new(); n];
    let mut zombie_towns = vec![false; n];
    let mut acc_fees = vec![inf; n];

    for _ in 0..k {
        sc.new_line();
        let tmp = sc.get::<usize>() - 1;
        zombie_towns[tmp] = true;
    }

    for _ in 0..m {
        sc.new_line();
        let c = sc.get::<usize>() - 1;
        let d = sc.get::<usize>() - 1;
        edges[c].push(d);
        edges[d].push(c);
    }

    for zt in 0..zombie_towns.len() {
        if !zombie_towns[zt] {
            continue;
        }
        let mut que = VecDeque::new();
        let mut step = 1;
        que.push_back((zt, step));
        while !que.is_empty() {
            let (target, step) = que.pop_front().unwrap();
            if step > s {
                continue;
            }
            for &e in &edges[target] {
                if step < acc_fees[e] {
                    acc_fees[e] = step;
                    que.push_back((e, step + 1))
                }
            }
        }
    }

    for i in 0..acc_fees.len() {
        if acc_fees[i] != inf {
            acc_fees[i] = q;
        } else {
            acc_fees[i] = p;
        }
    }
    acc_fees[0] = 0;
    acc_fees[n - 1] = 0;

    // dijkstra

    let mut costs = vec![inf; n];
    costs[0] = 0;

    let mut heap: BinaryHeap<Reverse<Edge>> = BinaryHeap::new();
    heap.push(Reverse(Edge { to: 0, dist: 0 }));

    while !&heap.is_empty() {
        let Edge {
            to: from,
            dist: next_dist,
        } = heap.pop().unwrap().0;
        for &n in &edges[from as usize] {
            if zombie_towns[n] {
                continue;
            }
            if costs[n] > costs[from as usize] + acc_fees[n] {
                costs[n] = costs[from as usize] + acc_fees[n];
                heap.push(Reverse(Edge {
                    to: n,
                    dist: acc_fees[n] as i64,
                }));
            }
        }
    }
    println!("{}", costs[n - 1])
}
