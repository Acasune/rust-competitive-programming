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
        N:usize, K:usize,
        A:[i64;N]
    }
    let md = md1000000007;
    if K == 1 {
        let a = A.into_iter().max().unwrap();
        println!("{}", (a + md) % md);
        return;
    }
    let mut negs = vec![];
    let mut poss = vec![];
    for a in A {
        if a < 0 {
            negs.push(a);
        } else {
            poss.push(a);
        }
    }
    negs.sort();
    poss.sort();

    let mut all = negs
        .iter()
        .chain(poss.iter())
        .cloned()
        .collect::<Vec<i64>>();

    if poss.len() == 0 {
        // neg pattern 1
        let mut ans = 1;
        if K % 2 == 1 {
            negs.reverse();
        }
        for i in 0..K {
            ans *= negs[i];
            ans = (md + ans) % md;
        }
        println!("{}", (md + ans) % md);
    } else if poss.len() <= 1 && negs.len() == 1 {
        println!("{}", (md + poss[0] * negs[0]) % md);
    } else {
        let mut cums = vec![1];
        for i in 0..N {
            cums.push((md + cums[i] * all[i]) % md);
        }

        let mut l = K as i64;
        let mut r = (N) as i64;
        if K % 2 == 1 {
            r -= 1;
            l = K as i64 - 1;
        }

        while 0 <= l - 2 as i64
            && all[l as usize - 1] * all[l as usize - 2]
                <= all[r as usize - 1] * all[r as usize - 2]
        {
            l -= 2;
            r -= 2;
        }
        if l <= 0 {
            let mut ans = 1;
            for i in (N - K)..N {
                ans *= all[i];
                ans = (md + ans) % md;
            }
            println!("{}", (md + ans) % md);
        } else {
            let mut ans = 1;
            for i in 0..(l as usize) {
                ans *= all[i];
                ans = (md + ans) % md;
            }
            for i in (r as usize)..N {
                ans *= all[i];
                ans = (md + ans) % md;
            }
            println!("{}", (md + ans) % md);
        }
    }
}
