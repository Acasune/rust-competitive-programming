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

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        S:Chars,
    }
    let mut l = 0;
    let mut r = S.len() - 1;
    while S[r] == 'a' && l < r {
        if S[l] == 'a' {
            l += 1;
            r -= 1;
        } else {
            r -= 1;
        }
    }
    if l >= r {
        println!("{}", "Yes");
        return;
    }
    while l < r {
        if S[l] == S[r] {
            l += 1;
            r -= 1;
        } else {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
    return;
}