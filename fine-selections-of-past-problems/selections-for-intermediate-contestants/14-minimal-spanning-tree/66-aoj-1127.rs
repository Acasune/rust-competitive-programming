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

#[derive(Copy, Clone)]
struct Edge {
    dist: f64,
    from: usize,
    to: usize,
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

    loop {
        sc.new_line();
        let mut n = sc.get::<usize>();
        if n == 0 {
            return;
        }

        let mut cells = vec![];
        for _ in 0..n {
            sc.new_line();
            let mut a = sc.get_as_vec::<f64>();
            cells.push(a);
        }

        let mut edges = vec![];

        for i in 0..n {
            for j in i + 1..n {
                let tmp = ((cells[i][0] - cells[j][0]) * (cells[i][0] - cells[j][0])
                    + (cells[i][1] - cells[j][1]) * (cells[i][1] - cells[j][1])
                    + (cells[i][2] - cells[j][2]) * (cells[i][2] - cells[j][2]))
                    .sqrt();
                let lad = {
                    if tmp <= cells[i][3] + cells[j][3] {
                        0.
                    } else {
                        tmp - (cells[i][3] + cells[j][3])
                    }
                };
                edges.push(Edge {
                    from: i,
                    to: j,
                    dist: lad,
                })
            }
        }
        edges.sort_by(|a, b| a.dist.partial_cmp(&b.dist).unwrap());

        let mut uft = UnionFindTree::new(n);
        let mut ans = 0.;

        for e in &edges {
            if uft.is_same(e.from as isize, e.to as isize) {
                continue;
            }
            ans += e.dist;

            uft.unite(e.from as isize, e.to as isize);
        }

        println!("{:0.3}", ans);
    }
}
