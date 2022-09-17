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
const d8yx: [[i64; 8]; 2] = [[1, 1, 0, -1, -1, -1, 0, 1], [0, 1, 1, 1, 0, -1, -1, -1]];
const d4yx: [[i64; 4]; 2] = [[1, 0, -1, 0], [0, 1, 0, -1]];

fn main() {
    input! {
        N:usize,C:usize,
        TA:[(usize,usize);N]
    }
    let mut ans = vec![0; N];
    for i in 0..30 {
        let mut func = vec![0, 1];
        let mut cur = bit(C, i);
        for (j, &(t, a)) in TA.iter().enumerate() {
            let mut f = vec![];
            let x = bit(a, i);
            if t == 1 {
                f = vec![x & 0, x & 1];
            }
            if t == 2 {
                f = vec![x | 0, x | 1];
            }
            if t == 3 {
                f = vec![x ^ 0, x ^ 1];
            }
            func = vec![f[func[0]], f[func[1]]];
            cur = func[cur];
            ans[j] |= cur << i;
        }
    }
    for a in ans {
        println!("{}", a);
    }
}

fn bit(x: usize, i: usize) -> usize {
    (x >> i) & 1
}
