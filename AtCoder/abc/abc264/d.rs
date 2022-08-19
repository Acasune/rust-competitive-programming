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
        mut S:Chars
    }
    let atcoder = "atcoder".to_string().chars().collect::<Vec<char>>();
    let mut cnt = 0;
    loop {
        if S == atcoder {
            break;
        }
        for i in 0..6 {
            if S[i] == 'a' {
                continue;
            }
            if S[i] == 't' && !(S[i + 1] == 'a') {
                continue;
            }
            if S[i] == 'c' && !(S[i + 1] == 'a' || S[i + 1] == 't') {
                continue;
            }
            if S[i] == 'o' && !(S[i + 1] == 'a' || S[i + 1] == 't' || S[i + 1] == 'c') {
                continue;
            }
            if S[i] == 'd'
                && !(S[i + 1] == 'a' || S[i + 1] == 't' || S[i + 1] == 'c' || S[i + 1] == 'o')
            {
                continue;
            }
            if S[i] == 'e'
                && !(S[i + 1] == 'a'
                    || S[i + 1] == 't'
                    || S[i + 1] == 'c'
                    || S[i + 1] == 'o'
                    || S[i + 1] == 'd')
            {
                continue;
            }
            cnt += 1;
            S.swap(i, i + 1);
        }
    }
    println!("{}", cnt);
}
