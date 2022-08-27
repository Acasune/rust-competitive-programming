#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::convert::TryInto;
use std::{char, f32, f64, i32, i64, usize};
use superslice::*;

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
    let mut E = vec![vec![]; N];
    for (i, &(a, b, c)) in ABC.iter().enumerate() {
        E[a].push((b, c, i));
        E[b].push((a, c, i));
    }
    let paths = dijkstra(0, &E);
    for d in paths {
        print!("{} ", d + 1);
    }
    println!();
}

fn dijkstra(s: usize, E: &Vec<Vec<(usize, usize, usize)>>) -> Vec<usize> {
    let mut paths = vec![];

    let N = E.len();
    let mut dists = vec![usize::MAX / 10; N];
    dists[s] = 0;
    let mut que = BinaryHeap::<Reverse<(usize, usize)>>::new();
    que.push(Reverse((0, s)));
    while let Some(Reverse((c, s))) = que.pop() {
        if dists[s] < c {
            continue;
        }
        for &(nxt, dist, i) in &E[s] {
            if dists[nxt] > dists[s] + dist {
                dists[nxt] = dists[s] + dist;
                que.push(Reverse((dists[nxt], nxt)));
            }
        }
    }

    let mut dists2 = vec![usize::MAX / 10; N];
    dists2[s] = 0;

    let mut que = BinaryHeap::<Reverse<(usize, usize)>>::new();
    que.push(Reverse((0, s)));
    while let Some(Reverse((c, s))) = que.pop() {
        if dists2[s] < c {
            continue;
        }
        for &(nxt, dist, i) in &E[s] {
            if dists2[nxt] > dists2[s] + dist {
                if dists2[s] + dist == dists[nxt] {
                    paths.push(i);
                }
                dists2[nxt] = dists2[s] + dist;
                que.push(Reverse((dists2[nxt], nxt)));
            }
        }
    }
    return paths;
}
