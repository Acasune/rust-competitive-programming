#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        Q:usize
    }
    let mut q2 = vec![];
    let mut set = vec![];
    let mut cnt = 0;
    for q in 0..Q {
        input! {op:usize}
        if op == 2 {
            input! {c:usize}
            q2.push(c);
        } else {
            input! {x:usize,c:usize}
            set.push(vec![cnt + c, x]);
            cnt += c;
        }
    }

    let mut base = 0usize;
    let mut idx = 0;
    for q in q2 {
        let upper = base + q;
        let mut ans = 0;
        loop {
            if set[idx][0] >= upper {
                ans += set[idx][1] * (upper - base);
                // set[idx][0] =;
                base = upper;
                break;
            }
            ans += set[idx][1] * (set[idx][0] - base);
            base = set[idx][0];
            idx += 1;
        }
        println!("{}", ans);
    }
}
