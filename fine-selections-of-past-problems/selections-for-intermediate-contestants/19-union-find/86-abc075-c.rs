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

    let mut ans = 0;

    for i in 0..m {
        let mut uft = UnionFindTree::new(n);
        for j in 0..m {
            if i == j {
                continue;
            }
            uft.unite(edges[j][0], edges[j][1]);
        }
        let mut flg = true;
        for k in 0..n {
            for l in k + 1..n {
                flg = flg & uft.is_same(k as isize, l as isize);
            }
        }
        if !flg {
            ans += 1;
        }
    }
    println!("{}", ans);
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
