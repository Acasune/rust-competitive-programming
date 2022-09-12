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
    let mut ans = vec![];
    let mn = C.iter().cloned().min().unwrap();
    let mut len = N / mn;
    let mut rem = N % mn;
    while len > 0 {
        // println!("{:?} {}", ans, rem);
        for i in (0..9).rev() {
            let c = C[i];
            if rem >= c - mn {
                rem -= c - mn;
                ans.push(char::from_digit((i + 1) as u32, 10).unwrap());
                break;
            }
        }
        len -= 1;
    }

    println!("{}", ans.into_iter().collect::<String>());
}
