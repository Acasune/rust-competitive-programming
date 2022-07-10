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
const d8yx: [[i64; 8]; 2] = [[1, 1, 0, -1, -1, -1, 0, 1], [0, 1, 1, 1, 0, -1, -1, -1]];
const d4yx: [[i64; 4]; 2] = [[1, 0, -1, 0], [0, 1, 0, -1]];

fn main() {
    input! {
        N:usize,
        sx:i64,sy:i64,tx:i64,ty:i64,
        syr:[(i64,i64,i64);N]
    }
    let mut G = vec![vec![]; N];
    for i in 0..N {
        let fst = syr[i];
        for j in 0..N {
            if i == j {
                continue;
            }
            let snd = syr[j];
            if (fst.0 - snd.0) * (fst.0 - snd.0) + (fst.1 - snd.1) * (fst.1 - snd.1)
                <= (fst.2 + snd.2) * (fst.2 + snd.2)
            {
                if (fst.0 - snd.0) * (fst.0 - snd.0) + (fst.1 - snd.1) * (fst.1 - snd.1)
                    >= (fst.2 - snd.2) * (fst.2 - snd.2)
                {
                    G[i].push(j);
                }
            }
        }
    }
    // println!("{:?}", G);
    let mut starts = vec![];
    let mut goals = vec![];
    for i in 0..N {
        let (x, y, r) = syr[i];
        if (x - sx) * (x - sx) + (y - sy) * (y - sy) == r * r {
            starts.push(i);
        }
        if (x - tx) * (x - tx) + (y - ty) * (y - ty) == r * r {
            goals.push(i);
        }
    }
    // println!("{:?}", goals);
    let gls: HashSet<usize> = goals.into_iter().collect::<HashSet<usize>>();
    let mut flg = false;

    for st in starts {
        if flg {
            break;
        }
        let mut visited = vec![false; N];
        let mut que = VecDeque::<usize>::new();
        visited[st] = true;
        if gls.contains(&st) {
            flg = true;
        }
        que.push_back(st);
        while let Some(sa) = que.pop_front() {
            if gls.contains(&sa) {
                flg = true;
            }

            for &nxt in &G[sa] {
                if !visited[nxt] {
                    visited[nxt] = true;
                    que.push_back(nxt);
                }
            }
        }
    }
    if flg {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}
