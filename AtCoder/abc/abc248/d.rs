#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        N:usize,
        A:[usize;N],
        Q:usize,
        ask:[(usize,usize,usize);Q]
    }
    let mut vec = vec![];
    for i in 0..N {
        vec.push((A[i], i + 1));
    }
    vec.push((0, 0));
    vec.push((inf as usize, inf as usize));
    vec.sort();
    // println!("{:?}", vec);
    for (lin, rin, x) in ask {
        let mut l = 0;
        let mut r = vec.len() + 1;
        let mut m = 0;
        let mut lidx = 0;
        while r - l > 1 {
            m = (r + l) / 2;
            if vec[m].0 < x {
                l = m;
            } else if vec[m].0 > x {
                r = m;
            } else {
                if vec[m].1 >= lin {
                    r = m;
                } else {
                    l = m;
                }
            }
        }
        lidx = r;
        let mut l = 0;
        let mut r = vec.len() + 1;
        let mut m = 0;
        let mut ridx = 0;
        let mut rin = rin;
        while r - l > 1 {
            m = (r + l) / 2;
            if vec[m].0 < x {
                l = m;
            } else if vec[m].0 > x {
                r = m;
            } else {
                if vec[m].1 > rin {
                    r = m;
                } else {
                    l = m;
                }
            }
        }
        ridx = r;
        // println!("{} {}", lidx + 1, ridx + 1);
        println!("{}", ridx - lidx);
    }
}
