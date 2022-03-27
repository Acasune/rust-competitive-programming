#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
       mut S:Chars,
       mut T:Chars,
    }
    for i in 0..(S.len() - 1) {
        if S[i] != T[i] {
            let c = S[i];
            S[i] = S[i + 1];
            S[i + 1] = c;
            break;
        }
    }
    for i in 0..S.len() {
        if S[i] != T[i] {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}
