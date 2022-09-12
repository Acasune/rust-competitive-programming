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
        N:usize,
        xyp:[(i64,i64,i64);N],
    }
    let mut l = -1;
    let mut r = 1 << 33;
    while r > l + 1 {
        let m = r + l >> 1;
        // println!("{}", m);
        let mut flg = false;
        for s in 0..N {
            let mut visited = vec![false; N];
            visited[s] = true;
            let mut que = VecDeque::<usize>::new();
            que.push_back(s);
            while let Some(s) = que.pop_front() {
                let (sx, sy, sp) = xyp[s];
                for t in 0..N {
                    if visited[t] {
                        continue;
                    }
                    let (tx, ty, _) = xyp[t];
                    if m * sp >= i64::abs(tx - sx) + i64::abs(ty - sy) {
                        visited[t] = true;
                        que.push_back(t);
                    }
                }
            }
            if visited.into_iter().all(|e| e) {
                flg = true;
                break;
            }
        }
        if flg {
            r = m;
        } else {
            l = m;
        }
    }
    println!("{}", r);
}
