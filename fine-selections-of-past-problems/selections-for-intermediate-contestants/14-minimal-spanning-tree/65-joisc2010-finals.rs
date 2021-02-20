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
    dist: i64,
    from: usize,
    to: usize,
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

trait UnionFind {
    fn new(n: usize) -> UnionFindTree;
    fn root(&mut self, x: isize) -> isize;
    fn is_same(&mut self, x: isize, y: isize) -> bool;
    fn unite(&mut self, x: isize, y: isize);
}
struct UnionFindTree {
    par: Vec<isize>,
    size: Vec<isize>,
}

impl UnionFind for UnionFindTree {
    fn new(n: usize) -> UnionFindTree {
        UnionFindTree {
            par: vec![-1; n],
            size: vec![1; n],
        }
    }
    fn root(&mut self, x: isize) -> isize {
        match self.par[x as usize] {
            -1 => x,
            _ => {
                self.par[x as usize] = self.root(self.par[x as usize]);
                self.par[x as usize]
            }
        }
    }

    fn is_same(&mut self, x: isize, y: isize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, x: isize, y: isize) {
        let mut root_x: isize = self.root(x);
        let mut root_y: isize = self.root(y);
        if root_x == root_y {
            return;
        }

        if self.size[root_x as usize] < self.size[root_y as usize] {
            root_x ^= root_y;
            root_y ^= root_x;
            root_x ^= root_y;
        }

        self.par[root_y as usize] = root_x;
        self.size[root_x as usize] += self.size[root_y as usize]
    }
}

fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let mut v = sc.get::<usize>();
    let mut e = sc.get::<usize>();
    let mut k = sc.get::<usize>();
    let inf = 1_000_000_010;
    let mut edges = vec![];

    for _ in 0..e {
        sc.new_line();
        let mut a = sc.get_as_vec::<usize>();
        edges.push(Edge {
            from: a[0] - 1,
            to: a[1] - 1,
            dist: a[2] as i64,
        });
    }

    let mut ans = 0;

    edges.sort();

    let mut uft = UnionFindTree::new(v);

    let mut cut_set = vec![];

    for i in 0..e {
        let e = edges[i];
        let from = e.from;
        let to = e.to;
        let dist = e.dist;

        if uft.is_same(from as isize, to as isize) {
            continue;
        }
        ans += dist;
        cut_set.push(dist);
        uft.unite(from as isize, to as isize);
    }
    cut_set.sort();
    cut_set.reverse();
    let mut cut_cuts = 0;
    for i in 0..k - 1 {
        cut_cuts += cut_set[i];
    }

    println!("{}", ans - cut_cuts);
}
