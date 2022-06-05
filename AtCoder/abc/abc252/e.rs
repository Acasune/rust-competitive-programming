#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i32, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:usize,M:usize,
        ABC:[(Usize1,Usize1,usize);M],
    }
    let mut edges = vec![vec![]; N];
    for (i, (a, b, c)) in ABC.into_iter().enumerate() {
        edges[a].push((c, b, i + 1));
        edges[b].push((c, a, i + 1));
    }
    // let mut dists = vec![inf_u; N];
    // dists[0] = 0;
    let mut ans = dijkstra(0, &edges);
    print!("{}", ans[0]);
    for i in 1..ans.len() {
        print!(" {}", ans[i]);
    }
    println!();
}

fn dijkstra(s: usize, E: &Vec<Vec<(usize, usize, usize)>>) -> Vec<usize> {
    let mut ans = vec![];
    let N = E.len();
    let mut dists = vec![usize::MAX / 10; N];
    dists[s] = 0;
    let mut que = BinaryHeap::<Reverse<(usize, usize, usize)>>::new();
    que.push(Reverse((0, s, 0)));
    while let Some(Reverse((c, s, i))) = que.pop() {
        if dists[s] < c {
            continue;
        }

        for &(dist, nxt, i) in &E[s] {
            if dists[nxt] > dists[s] + dist {
                dists[nxt] = dists[s] + dist;
                que.push(Reverse((dist, nxt, i)));
            }
        }
    }
    let mut dists2 = vec![usize::MAX / 10; N];
    dists2[s] = 0;
    let mut que = BinaryHeap::<Reverse<(usize, usize, usize)>>::new();
    que.push(Reverse((0, s, 0)));
    while let Some(Reverse((c, s, i))) = que.pop() {
        if dists2[s] < c {
            continue;
        }
        // ans.push(i);

        for &(dist, nxt, i) in &E[s] {
            if dists2[nxt] > dists2[s] + dist {
                dists2[nxt] = dists2[s] + dist;
                if dists2[nxt] == dists[nxt] {
                    ans.push(i);
                }
                que.push(Reverse((dist, nxt, i)));
            }
        }
    }
    return ans;
}
