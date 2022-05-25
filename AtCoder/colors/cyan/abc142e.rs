#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use itertools::Itertools;
use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
         N:usize,M:usize,
    }
    let mut prices = vec![vec![]; M];
    let mut price_list = vec![];
    for m in 0..M {
        input! {
             a:usize,b:usize,
             c:[Usize1;b]
        }
        prices[m].extend(c);
        price_list.push(a);
    }
    let mut states = vec![inf as usize; 1 << N];
    states[0] = 0;
    for state in 0..1 << N {
        for i in 0..M {
            let mut bit = 0;
            for &j in &prices[i] {
                bit |= 1 << j;
            }
            let price = states[state] + price_list[i];
            if price < states[bit | state] {
                states[bit | state] = price;
            }
        }
    }
    let ans = if states[(1 << N) - 1] >= inf as usize {
        -1
    } else {
        states[(1 << N) - 1] as i64
    };
    println!("{}", ans);
}
