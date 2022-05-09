#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:usize,Q:usize,
        X:[Usize1;Q],
    }
    let mut pos1 = vec![]; //from pos to val
    let mut pos2 = vec![]; // from val to pos
    for i in 0..N {
        pos1.push(i);
        pos2.push(i);
    }
    for x in X {
        let pos = pos2[x];
        let mut next_pos = 0;
        if pos < N - 1 {
            next_pos = pos + 1;
        } else {
            next_pos = pos - 1;
        }
        let target = pos1[next_pos];
        pos1.swap(pos, next_pos);
        pos2.swap(x, target);
    }
    print!("{}", pos1[0] + 1);
    for i in 1..N {
        print!(" {}", pos1[i] + 1);
    }
    println!("");
}
