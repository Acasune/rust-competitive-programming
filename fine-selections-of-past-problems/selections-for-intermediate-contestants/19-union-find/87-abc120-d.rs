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
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n = sc.get::<usize>();
    let m = sc.get::<usize>();
    let mut edges = vec![];
    for i in 0..m {
        sc.new_line();
        let mut a = sc.get_as_vec::<isize>();
        a[0] -= 1;
        a[1] -= 1;
        edges.push(a);
    }
    edges.reverse();

    let mut ans = vec![];

    let mut uft = UnionFindTree::new(n);
    ans.push(uft.cnt);

    for e in edges {
        ans.push(uft.unite(e[0], e[1]));
    }
    ans.reverse();
    for i in 0..m {
        println!("{}", ans[i + 1]);
    }
}

trait UnionFind {
    fn new(n: usize) -> UnionFindTree;
    fn root(&mut self, x: isize) -> isize;
    fn is_same(&mut self, x: isize, y: isize) -> bool;
    fn unite(&mut self, x: isize, y: isize) -> isize;
}
struct UnionFindTree {
    par: Vec<isize>,
    size: Vec<isize>,
    cnt: isize,
}

impl UnionFind for UnionFindTree {
    fn new(n: usize) -> UnionFindTree {
        UnionFindTree {
            par: vec![-1; n],
            size: vec![1; n],
            cnt: n as isize * (n as isize - 1) / 2,
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

    fn unite(&mut self, x: isize, y: isize) -> isize {
        let mut root_x: isize = self.root(x);
        let mut root_y: isize = self.root(y);
        if root_x == root_y {
            return self.cnt;
        }

        if self.size[root_x as usize] < self.size[root_y as usize] {
            root_x ^= root_y;
            root_y ^= root_x;
            root_x ^= root_y;
        }
        self.cnt -= self.size[root_x as usize] * self.size[root_y as usize];

        self.par[root_y as usize] = root_x;
        self.size[root_x as usize] += self.size[root_y as usize];

        return self.cnt;
    }
}
