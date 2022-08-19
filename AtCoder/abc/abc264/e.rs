#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{char, f32, f64, i32, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const d8yx: [(i64, i64); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];
const d4yx: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    input! {
        N:usize,M:usize, E:usize,
        UV: [(Usize1,Usize1);E],
        Q:usize,
        mut X:[Usize1;Q],
    }
    let mut x_set = HashSet::from(X.iter().cloned().collect::<HashSet<usize>>());
    let mut node = vec![1; N + M];
    let mut has_light = vec![false; N + M];

    for i in N..N + M {
        node[i] = 0;
        has_light[i] = true;
    }

    let mut uft = UnionFindTree::new(N + M);
    for i in 0..E {
        if x_set.contains(&i) {
            continue;
        }
        let (u, v) = UV[i];
        if N <= u {
            continue;
        }
        if v < N {
            let g_u = uft.find(u);
            let g_v = uft.find(v);
            let cnt = node[g_u] + node[g_v];
            let is_light = has_light[g_u] || has_light[g_v];
            if g_u == g_v {
                continue;
            }
            uft.unite(g_u, g_v);
            let g_x = uft.find(g_u);
            node[g_x] = cnt;
            has_light[g_x] |= is_light;
        } else {
            let g_u = uft.find(u);
            let g_v = uft.find(v);
            let cnt = node[g_u] + node[g_v];
            if g_u == g_v {
                continue;
            }
            uft.unite(g_u, g_v);
            let g_x = uft.find(g_u);
            node[g_x] = cnt;
            has_light[g_x] |= true;
        }
    }
    let mut cnt = 0;
    let mut visited = HashSet::new();
    for i in 0..N {
        let root = uft.find(i);
        if visited.contains(&root) {
            continue;
        }

        if has_light[root] {
            cnt += node[root];
        }
        visited.insert(root);
    }

    let mut ans = vec![];
    ans.push(cnt);

    X.reverse();
    for x in X {
        let (u, v) = UV[x];
        let g_u = uft.find(u);
        let g_v = uft.find(v);
        if g_u == g_v {
        } else {
            if !has_light[g_u] && has_light[g_v] {
                cnt += node[g_u];
                uft.unite(g_u, g_v);
                let g_x = uft.find(g_u);
                has_light[g_x] = true;
            } else if has_light[g_u] && !has_light[g_v] {
                cnt += node[g_v];
                uft.unite(g_u, g_v);
                let g_x = uft.find(g_u);
                has_light[g_x] = true;
            } else {
                uft.unite(g_u, g_v);
                let g_x = uft.find(g_u);
                node[g_x] = node[g_u] + node[g_v];
            }
        }
        ans.push(cnt);
    }
    ans.reverse();
    for a in 1..ans.len() {
        println!("{}", ans[a]);
    }
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
