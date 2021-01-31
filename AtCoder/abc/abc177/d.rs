use std::{cmp::{max, min}, io::*, str::FromStr, usize};

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


struct UnionFind {
    root: Vec<usize>,
    rank: Vec<usize>,
    sizes: Vec<usize>,
    n: usize,
}
 
impl UnionFind {
    fn new(n: usize) -> UnionFind {
        let mut vec = vec![0;n];
        for i in 0..n {
            vec[i] = i;
        }
        UnionFind {
            root: vec,
            rank: vec![0;n],
            sizes: vec![1;n],
            n,
        }
    }
 
    fn find(&mut self, x: usize) -> usize {
        if self.root[x] == x { return x }
        self.root[x] = self.find(self.root[x]);
        self.root[x]
    }
 
    fn unite(&mut self, x: usize, y: usize) -> () {
        let x = self.find(x);
        let y = self.find(y);
        if x == y { return }
        if self.rank[x] < self.rank[y] {
            self.root[x] = y;
            self.sizes[y] += self.sizes[x];
        } else {
            self.root[y] = x;
            self.sizes[x] += self.sizes[y];
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }
 
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
 
    fn size(&mut self, x: usize) -> usize{
        let f = self.find(x);
        self.sizes[f]
    }
 
    fn roots(&mut self) -> Vec<usize> {
        let mut vec = vec![];
        for i in 0..self.n {
            if self.root[i] == i {
                vec.push(self.root[i]);
            }
        }
        vec
    }
}
fn main() {
    let cin = stdin();
    let cin = cin.lock();
    let mut sc = Scanner::new(cin);

    sc.new_line();
    let n:usize = sc.get();
    let m:usize = sc.get();

    let mut uf = UnionFind::new(n);

    for i in 0..m {
        sc.new_line();
        let mut v:Vec<usize> = sc.get_as_vec();
        uf.unite(v[0]-1, v[1]-1);
    }
    let mut ans:i64 =-1;

    for i in 0..n {
        ans = max(ans,uf.size(i) as i64);
    }



    println!("{}",ans);


}
