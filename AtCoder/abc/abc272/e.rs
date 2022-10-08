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

const inf_i: i64 = (i64::MAX / 10) * 9;
const inf_u: usize = (usize::MAX / 10) * 9;
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
        N:usize,M:i64,
        mut A:[i64;N],
    }
    let mut memo = vec![vec![]; M as usize + 2];
    for i in 0..N {
        let mut a = A[i];
        if a > N as i64 {
            continue;
        }
        let l = if a >= 0 {
            1
        } else {
            (-a + i as i64) / (i as i64 + 1)
        };
        let r = i64::min(M + 1, (N as i64 - a + i as i64) / (i as i64 + 1));

        for j in l..=r {
            memo[j as usize].push(a + j * (i as i64 + 1));
        }
    }
    for m in 1..=M as usize {
        let mem = &mut memo[m];
        mem.sort();
        mem.dedup();
        let m = mem.len();
        let mut flg = false;
        for i in 0..m {
            if i as i64 != mem[i] {
                println!("{}", i);
                flg = true;
                break;
            }
        }
        if !flg {
            println!("{}", m);
        }
    }
}
