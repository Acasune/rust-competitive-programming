#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, BTreeMap};
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
        Q:usize,
    }

    let mut mp = BTreeMap::new();
    for _ in 0..Q {
        input! {query:usize}
        if query == 1 {
            input! {x:usize}
            *mp.entry(x).or_insert(0)+=1;
        } else if query == 2 {
            input! {x:usize,mut c:usize}
            if let Some(&cnt) = mp.get(&x) {
                if cnt > c {
                    *mp.get_mut(&x).unwrap() -= c;
                } else {
                    mp.remove(&x);
                }
            }
        }
        else {
            let min = mp.iter().next().unwrap().0;
            let max = mp.iter().next_back().unwrap().0;
            println!("{}",max-min);
        }
    }
}
