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
        S:Chars,
    }
    if S[0] == '1' {
        println!("{}", "No");
        return;
    }
    let mut is_stand = vec![false; 7];
    if S[6] == '1' {
        is_stand[0] = true;
    }
    if S[3] == '1' {
        is_stand[1] = true;
    }
    if S[1] == '1' || S[7] == '1' {
        is_stand[2] = true;
    }
    if S[4] == '1' {
        is_stand[3] = true;
    }
    if S[2] == '1' || S[8] == '1' {
        is_stand[4] = true;
    }
    if S[5] == '1' {
        is_stand[5] = true;
    }
    if S[9] == '1' {
        is_stand[6] = true;
    }
    for i in 0..7 {
        for j in i + 2..7 {
            // at least one false
            let mut flg = false;
            for k in i + 1..j {
                flg |= !is_stand[k];
            }
            if is_stand[i] && flg && is_stand[j] {
                println!("{}", "Yes");
                return;
            }
        }
    }
    println!("{}", "No");
}
