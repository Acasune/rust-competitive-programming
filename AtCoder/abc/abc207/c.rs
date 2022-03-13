#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;

fn main() {
    input! {
         N:usize,
         TLR:[(i64,i64,i64);N],
    }
    let mut sections = vec![vec![inf, -inf]; N];
    for (i, &(t, l, r)) in TLR.iter().enumerate() {
        if t == 1 {
            sections[i][0] = 2 * l;
            sections[i][1] = 2 * r;
        } else if t == 2 {
            sections[i][0] = 2 * l;
            sections[i][1] = 2 * r - 1;
        } else if t == 3 {
            sections[i][0] = 2 * l + 1;
            sections[i][1] = 2 * r;
        } else {
            sections[i][0] = 2 * l + 1;
            sections[i][1] = 2 * r - 1;
        }
    }
    let mut ans = 0i64;
    for i in 0..N {
        for j in i + 1..N {
            let l1 = sections[i][0];
            let r1 = sections[i][1];
            let l2 = sections[j][0];
            let r2 = sections[j][1];
            if l1 <= l2 && l2 <= r1
                || l1 <= r2 && r2 <= r1
                || l2 <= l1 && l1 <= r2
                || l2 <= r1 && r1 <= r2
            {
                ans += 1;
                // println!("{} {}", i, j);
            }
        }
    }
    println!("{}", ans);
}
