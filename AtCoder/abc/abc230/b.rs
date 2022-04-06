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
        S:Chars
    }
    let n = S.len();
    let mut vec = vec!['a'; 1_00000];
    for i in 0..1_00000 {
        if i % 3 == 0 {
            vec[i] = 'o';
        } else {
            vec[i] = 'x';
        }
    }
    for i in 0..(1_00000 - n) {
        let mut flg = true;
        for j in 0..n {
            if vec[i + j] != S[j] {
                flg = false;
                break;
            }
        }
        if flg {
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");
}
