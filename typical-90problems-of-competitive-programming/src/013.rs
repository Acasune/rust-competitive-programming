use std::cmp::{max, min, Reverse};
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::{io::*, str::FromStr};

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

fn dijkstra(s: usize, N: usize, edges: &Vec<Vec<(usize, usize)>>) -> Vec<usize> {
    let mut dist = vec![100_000_000_000; N];
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0, s)));
    dist[s] = 0;

    while !heap.is_empty() {
        let Reverse((c, prev)) = heap.pop().unwrap();
        if dist[prev] < c {
            continue;
        }
        for i in 0..edges[prev].len() {
            let (nc, next) = edges[prev][i];
            if c + nc < dist[next] {
                dist[next] = c + nc;
                heap.push(Reverse((nc + c, next)));
            }
        }
    }
    return dist;
}
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let N: usize = sc.get();
    let M: usize = sc.get();
    let mut edges = vec![Vec::new(); N];
    for _ in 0..M {
        sc.new_line();
        let a: usize = sc.get();
        let b: usize = sc.get();
        let c: usize = sc.get();
        edges[a - 1].push((c, b - 1));
        edges[b - 1].push((c, a - 1));
    }
    let dist1 = dijkstra(0, N, &edges);
    let dist2 = dijkstra(N - 1, N, &edges);
    for i in 0..N {
        println!("{}", dist1[i] + dist2[i]);
    }
}
