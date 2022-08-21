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
        N:usize,M:usize,mut T:i64,
        A:[i64;N-1],
        XY:[(Usize1,i64);M]
    }
    let mut mp = HashMap::new();
    for (x, y) in XY {
        mp.insert(x, y);
    }
    for i in 0..N - 1 {
        if let Some(val) = mp.get(&i) {
            T += val;
        }
        T -= A[i];
        if T <= 0 {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}
