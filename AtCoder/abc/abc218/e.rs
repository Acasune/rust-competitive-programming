#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,M:usize,
        ABC:[(Usize1,Usize1,i64);M],
    }
    let mut abc = vec![];
    for (a, b, c) in ABC {
        abc.push((c, a, b));
    }
    abc.sort();
    let mut ans = 0;
    let mut tree = UnionFindTree::new(N);
    for (c, a, b) in abc {
        if !tree.is_same(a, b) || c <= 0 {
            tree.unite(a, b);
        } else {
            ans += c;
        }
    }
    println!("{}", ans);
}

struct UnionFindTree {
    par: Vec<usize>,
    rank: Vec<usize>,
    size: usize,
}
trait UnionFind {
    fn new(n: usize) -> UnionFindTree;
    fn find(&mut self, x: usize) -> usize;
    fn unite(&mut self, x: usize, y: usize);
    fn is_same(&mut self, x: usize, y: usize) -> bool;
}
impl UnionFind for UnionFindTree {
    fn new(n: usize) -> Self {
        let mut par = vec![0; n];
        let mut rank = vec![1; n];
        for i in 0..n {
            par[i] = i;
        }
        UnionFindTree {
            par: par,
            rank: rank,
            size: n,
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            self.par[x] = self.find(self.par[x]);
            self.par[x]
        }
    }
    fn unite(&mut self, x: usize, y: usize) {
        let mut x_root = self.find(self.par[x]);
        let mut y_root = self.find(self.par[y]);
        if x_root == y_root {
            return;
        }
        if self.rank[x_root] < self.rank[y_root] {
            x_root ^= y_root;
            y_root ^= x_root;
            x_root ^= y_root;
        }

        self.par[y_root] = x_root;
        self.rank[y_root] += self.rank[x_root];
    }
    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}
