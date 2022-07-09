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
        S:Chars,
        T:Chars
    }
    let mut s = vec![];
    let mut t = vec![];

    let mut l = 0;
    let mut r = 1;
    while r < S.len() {
        if S[l] != S[r] {
            s.push((S[l], r - l));
            l = r;
        }
        r += 1;
    }
    s.push((S[l], r - l));

    let mut l = 0;
    let mut r = 1;
    while r < T.len() {
        if T[l] != T[r] {
            t.push((T[l], r - l));
            l = r;
        }
        r += 1;
    }
    t.push((T[l], r - l));
    if s.len() != t.len() {
        println!("{}", "No");
        return;
    }
    for i in 0..s.len() {
        let (sc, sn) = s[i];
        let (tc, tn) = t[i];
        if sc != tc {
            println!("{}", "No");
            return;
        }
        if sn == 1 {
            if tn != 1 {
                println!("{}", "No");
                return;
            }
        } else if sn > tn {
            println!("{}", "No");
            return;
        }
    }
    println!("{}", "Yes");
}
