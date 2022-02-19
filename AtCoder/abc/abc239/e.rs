use std::{
    cmp::{max, min, Reverse},
    collections::{BinaryHeap, HashSet},
    io::*,
    str::FromStr,
};
use std::{collections::hash_set, iter::FromIterator};

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
struct UnionFindTree {
    par: Vec<usize>,
    rank: Vec<usize>,
    largest: Vec<usize>,
}

impl UnionFindTree {
    fn new(n: usize) -> Self {
        let mut tmp = vec![0; n];
        for i in 0..n {
            tmp[i] = i;
        }
        UnionFindTree {
            par: (0..n).collect::<Vec<usize>>(),
            rank: vec![0; n],
            largest: tmp,
        }
    }
    fn find_root(&mut self, a: usize) -> usize {
        if self.par[a] == a {
            return a;
        }
        self.par[a] = self.find_root(self.par[a]);
        return self.par[a];
    }
    fn is_unite(&mut self, a: usize, b: usize) -> bool {
        return self.find_root(a) == self.find_root(b);
    }
    fn unite(&mut self, a: usize, b: usize) {
        let ra = self.find_root(a);
        let rb = self.find_root(b);
        if ra == rb {
            return;
        }
        if self.largest[ra] < self.largest[rb] {
            self.largest[ra] = self.largest[rb];
        } else {
            self.largest[ra] = self.largest[rb];
        }
        if self.rank[ra] < self.rank[rb] {
            self.par[ra] = rb;
        } else {
            self.par[rb] = ra;
            if self.rank[ra] == self.rank[rb] {
                self.rank[ra] += 1;
            }
        }
    }
}
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let N = sc.get::<usize>();
    let Q = sc.get::<usize>();

    sc.new_line();
    let xl = sc.get_as_vec::<usize>();
    let mut edges = vec![Vec::<usize>::new(); N + 1];
    let mut ask = vec![Vec::<(usize, usize)>::new(); N];
    let mut ans = vec![0usize; Q];

    for i in 1..N {
        sc.new_line();
        let a = sc.get::<usize>() - 1;
        let b = sc.get::<usize>() - 1;
        edges[a].push(b);
        edges[b].push(a);
    }

    for i in 0..Q {
        sc.new_line();
        let v = sc.get::<usize>() - 1;
        let k = sc.get::<usize>();
        ask[v].push((i, k));
    }
    dfs(0, &edges, &ask, &mut ans, &xl, 1_000_000_000_000_000);
    for q in 0..Q {
        println!("{:?}", ans[q]);
    }
}

fn dfs(
    root: usize,
    edges: &Vec<Vec<usize>>,
    ask: &Vec<Vec<(usize, usize)>>,
    ans: &mut Vec<usize>,
    xl: &Vec<usize>,
    pre: usize,
) -> Vec<usize> {
    let mut a = vec![xl[root]];
    for &edge in &edges[root] {
        if pre == edge {
            continue;
        }

        a.append(&mut dfs(edge, edges, ask, ans, xl, root));
    }
    a.sort_by_key(|&a| !a);
    a.truncate(20);
    for &(i, k) in ask[root].iter() {
        ans[i] = a[k - 1];
    }
    a
}
