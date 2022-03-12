#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::{BinaryHeap, HashMap, HashSet};

use proconio::marker::*;
use proconio::*;
use std::{f64, i64};

fn main() {
    input! {
        N:usize,
        A:[usize;N],
    }
    let mut st = HashSet::<(usize, usize)>::new();
    let mut l = 0;
    let mut r = N - 1;
    while l < r {
        if A[l] != A[r] {
            st.insert((A[l], A[r]));
        }
        l += 1;
        r -= 1;
    }
    let mut ans = 0i64;
    let mut uf = UnionFindTree::new(2 * 100_010);
    for &(l, r) in st.iter() {
        if !uf.is_same(l, r) {
            ans += 1;
            uf.unite(l, r);
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
