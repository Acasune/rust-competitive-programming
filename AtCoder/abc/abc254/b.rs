#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{char,i32,f32,f64, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:usize,
    }
    let mut vec = vec![];
    vec.push(vec![1]);
    for i in 1..30 {
        let mut ve = vec![1i64];
        for j in 1..i {
            ve.push(vec[i-1][j-1]+vec[i-1][j]);
        }
        ve.push(1);
        vec.push(ve);
    }
    for j  in 0..N {
        let v = &vec[j];
        print!("{}",v[0]);
        for i in 1..v.len() {
            print!(" {}",v[i]);
        }
        println!();
    }
}
