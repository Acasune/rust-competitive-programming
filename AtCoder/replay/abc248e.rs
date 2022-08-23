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
        N:usize,K:usize,
        XY:[(i64,i64);N],
    }
    if K == 1 {
        println!("{}", "Infinity");
        return;
    }
    let mut used = vec![vec![false; 300]; 300];
    for i in 0..N {
        for j in i + 1..N {
            used[i][j] = true;
        }
    }
    let mut ans = 0i64;
    for i in 0..N {
        let (ax, ay) = XY[i];
        for j in i + 1..N {
            if used[i][j] {
                let (bx, by) = XY[j];
                let mut pnts = vec![i, j];
                let dx1 = ax - bx;
                let dy1 = ay - by;
                for k in j + 1..N {
                    let (cx, cy) = XY[k];
                    let dx2 = ax - cx;
                    let dy2 = ay - cy;
                    if dy1 * dx2 == dx1 * dy2 {
                        pnts.push(k);
                    }
                }
                let n = pnts.len();
                if n >= K {
                    ans += 1;
                    for i in 0..n {
                        for j in i + 1..n {
                            used[pnts[i]][pnts[j]] = false;
                        }
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
