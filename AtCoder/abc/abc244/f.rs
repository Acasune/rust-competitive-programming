#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use itertools::max;
use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{f64, i64, usize};

const inf: usize = usize::MAX / 10;
const md: i64 = 998244353;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,M:usize,
        UV:[(Usize1,Usize1);M],
    }
    let mut edges = vec![vec![]; N];
    for &(u, v) in &UV {
        edges[u].push(v);
        edges[v].push(u);
    }
    let mut dp = vec![vec![inf; N]; (1 << N)];
    let mut que = BinaryHeap::<Reverse<(usize, usize, usize)>>::new();
    for i in 0..N {
        dp[1 << (N - i - 1)][i] = 1;
        que.push(Reverse((dp[1 << (N - i - 1)][i], 1 << (N - i - 1), i)));
    }

    while let Some(Reverse((val, bit, from))) = que.pop() {
        for &e in &edges[from] {
            let next_bit = bit ^ (1 << (N - e - 1));
            if dp[next_bit][e] > dp[bit][from] + 1 {
                dp[next_bit][e] = dp[bit][from] + 1;
                que.push(Reverse((dp[next_bit][e], next_bit, e)));
            }
        }
    }
    let mut ans = 0;
    for i in 1..(1 << N) {
        let mut min = inf;
        for j in 0..N {
            min = min.min(dp[i][j]);
        }
        ans += min;
    }

    println!("{}", ans);
}
