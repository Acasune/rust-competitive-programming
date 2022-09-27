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

fn main() {
    input! {
        N:usize,K:usize,
        XY:[(i64,i64);N],
    }
    if K == 1 {
        println!("{}", "Infinity");
        return;
    }
    let mut ans = 0;
    let mut used = vec![vec![false; N]; N];
    for i in 0..N {
        let (ax, ay) = XY[i];
        for j in i + 1..N {
            if used[i][j] {
                continue;
            }
            let (bx, by) = XY[j];
            let mut cnt = 2;
            let mut visited = vec![i, j];
            for k in j + 1..N {
                let (cx, cy) = XY[k];
                if check(ax, ay, bx, by, cx, cy) {
                    cnt += 1;
                    visited.push(k);
                }
            }
            if cnt >= K {
                ans += 1;
            }
            for o in 0..visited.len() {
                for p in o + 1..visited.len() {
                    used[visited[o]][visited[p]] = true;
                    used[visited[p]][visited[o]] = true;
                }
            }
        }
    }
    println!("{}", ans);
}

fn check(ax: i64, ay: i64, bx: i64, by: i64, cx: i64, cy: i64) -> bool {
    let dx = ax - bx;
    let dy = ay - by;
    let ex = ax - cx;
    let ey = ay - cy;
    dx * ey == ex * dy
}
