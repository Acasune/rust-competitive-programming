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
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,
        mut S:[usize;N],
        mut T:[usize;N],
    }
    let mut flg = (0, T[0]);
    let mut i = 1;
    loop {
        let j = i % N;
        let k = (i - 1) % N;
        let prev = flg;
        if T[k] + S[k] < T[j] {
            T[j] = T[k] + S[k];
            flg = (j, T[j]);
        }
        if prev.0 == j && prev.1 == flg.1 {
            break;
        }
        i += 1;
    }

    for i in 0..N {
        println!("{}", T[i]);
    }
}
