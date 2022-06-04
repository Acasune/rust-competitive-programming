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
        N:usize,K:usize,
        mut A:[usize;N],
    }
    let mut arr = vec![BinaryHeap::<Reverse<usize>>::new();K];
    for i in 0..N {
        let a = A[i];
        arr[i%K].push(Reverse(a));
    }
    let mut cor = vec![];
    for i in 0..N {
        let Reverse(b) = arr[i%K].pop().unwrap();
        cor.push(b);
    }
    A.sort();
    // cor.reverse();
    // println!("{:?}",cor);
    if A == cor {
        println!("{}","Yes");
    } else {
        println!("{}","No");
    }

}
