#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{char, f32, f64, i32, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const d8yx: [[i64; 8]; 2] = [[1, 1, 0, -1, -1, -1, 0, 1], [0, 1, 1, 1, 0, -1, -1, -1]];
const d4yx: [[i64; 4]; 2] = [[1, 0, -1, 0], [0, 1, 0, -1]];

fn main() {
    input! {
        N:usize,M:usize,E:usize,
        UV:[(Usize1,Usize1);E],
        Q:usize,
       mut X:[Usize1;Q]
    }
    let st = X.iter().cloned().collect::<HashSet<usize>>();
    let mut uft = UnionFindTree::new(N + M);
    uft.initialize(N);

    let mut cnt = 0;
    for (i, &(u, v)) in UV.iter().enumerate() {
        if !st.contains(&i) {
            if uft.find(u) == uft.find(v) {
                continue;
            }
            let u_size = uft.find_node_size(u);
            let u_is_light = uft.is_light(u);
            let v_size = uft.find_node_size(v);
            let v_is_light = uft.is_light(v);
            uft.unite(u, v);
            if u_is_light && !v_is_light {
                cnt += v_size;
            } else if !u_is_light && v_is_light {
                cnt += u_size;
            }
        }
    }
    let mut ans = vec![];
    let last = cnt;
    X.reverse();
    for x in X {
        let u = UV[x].0;
        let v = UV[x].1;
        if uft.find(u) == uft.find(v) {
            ans.push(cnt);
            continue;
        }
        let u_size = uft.find_node_size(u);
        let u_is_light = uft.is_light(u);
        let v_size = uft.find_node_size(v);
        let v_is_light = uft.is_light(v);
        uft.unite(u, v);
        if u_is_light && !v_is_light {
            cnt += v_size;
        } else if !u_is_light && v_is_light {
            cnt += u_size;
        }
        ans.push(cnt);
    }
    ans.reverse();
    for i in 1..ans.len() {
        println!("{}", ans[i]);
    }
    println!("{}", last);
}
struct UnionFindTree {
    par: Vec<usize>,
    rank: Vec<usize>,
    size: usize,
    node_size: Vec<usize>,
    is_light: Vec<bool>,
}
trait UnionFind {
    fn new(n: usize) -> UnionFindTree;
    fn find(&mut self, x: usize) -> usize;
    fn unite(&mut self, x: usize, y: usize);
    fn is_same(&mut self, x: usize, y: usize) -> bool;
    fn initialize(&mut self, N: usize);
    fn find_node_size(&mut self, x: usize) -> usize;
    fn is_light(&mut self, x: usize) -> bool;
}
impl UnionFind for UnionFindTree {
    fn new(n: usize) -> Self {
        let mut par = vec![0; n];
        let mut rank = vec![1; n];
        let mut node_size = vec![0; n];
        let mut is_light = vec![false; n];
        for i in 0..n {
            par[i] = i;
        }
        UnionFindTree {
            par: par,
            rank: rank,
            size: n,
            node_size: node_size,
            is_light: is_light,
        }
    }
    fn initialize(&mut self, N: usize) {
        for i in 0..N {
            self.node_size[i] = 1;
        }
        for i in N..self.size {
            self.is_light[i] = true;
        }
    }
    fn find_node_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.node_size[root]
    }
    fn is_light(&mut self, x: usize) -> bool {
        let root = self.find(x);
        self.is_light[root]
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
        let is_light = self.is_light(x) || self.is_light(y);

        self.par[y_root] = x_root;
        self.rank[y_root] += self.rank[x_root];
        self.is_light[x_root] |= is_light;
        self.node_size[x_root] += self.node_size[y_root];
    }
    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}
