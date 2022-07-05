#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
use std::convert::TryInto;
use std::{char, f32, f64, i32, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:usize,M:usize, L:usize,
        ABC:[(Usize1,Usize1,usize);M],
        Q:usize,
        ST:[(Usize1,Usize1);Q]
    }
    let mut G = vec![vec![]; N];
    for (a, b, c) in ABC {
        if c <= L {
            G[a].push((b, c));
            G[b].push((a, c));
        }
    }
    let mut q = vec![vec![]; N];
    for (i, &(s, t)) in ST.iter().enumerate() {
        q[s].push((i, t));
    }
    let mut ans = vec![inf_i; Q];

    for i in 0..N {
        let mut que = BinaryHeap::<Reverse<((usize, usize), usize)>>::new();
        let mut dists = vec![(inf_u, inf_u); N];
        dists[i] = (0, 0);
        que.push(Reverse(((0, 0), i)));
        while let Some(Reverse(((cnt, d), s))) = que.pop() {
            if dists[s] < (cnt, d) {
                continue;
            }
            for &(t, c) in &G[s] {
                let mut nxt_cnt = 0;
                let mut nxt_d = 0;
                if c + d > L {
                    nxt_cnt = cnt + 1;
                    nxt_d = c;
                } else {
                    nxt_cnt = cnt;
                    nxt_d = c + d;
                }
                if dists[t] > (nxt_cnt, nxt_d) {
                    dists[t] = (nxt_cnt, nxt_d);
                    que.push(Reverse(((nxt_cnt, nxt_d), t)));
                }
            }
        }
        for &(j, t) in &q[i] {
            if dists[t].0 == inf_u {
                ans[j] = -1;
            } else {
                ans[j] = dists[t].0 as i64;
            }
        }
    }
    for a in ans {
        println!("{}", a);
    }
}
