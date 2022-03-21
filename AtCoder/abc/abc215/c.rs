#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use itertools::Itertools;
use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
       mut  S:Chars,K:usize,
    }
    let N = S.len();
    let mut st = HashSet::<String>::new();
    for s in S.into_iter().permutations(N) {
        st.insert(s.clone().into_iter().collect());
    }
    let mut vec = st.into_iter().collect::<Vec<String>>();
    vec.sort();

    println!("{}", vec[K - 1]);
}
