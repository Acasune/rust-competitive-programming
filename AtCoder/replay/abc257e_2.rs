#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::convert::TryInto;
use std::{char, f32, f64, i32, i64, usize};
use superslice::*;

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
       mut  N:i64,
        mut C:[i64;9],
    }
    C.insert(0, inf_i);
    let mn = C
        .iter()
        .cloned()
        .enumerate()
        .min_by_key(|&a| (a.1, Reverse(a.0)))
        .unwrap();
    let mut keta = N / mn.1;
    let mut rem = N % mn.1;
    let mut ans = vec![];
    while keta > 0 {
        for i in (mn.0..=9).rev() {
            if rem >= C[i] - mn.1 {
                rem -= C[i] - mn.1;
                ans.push(i);

                break;
            }
        }
        keta -= 1;
    }
    for a in ans {
        print!("{}", a);
    }
    println!()
}
