#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,
        mut XY:[(usize,usize);N],
    }
    XY.sort();
    let mut heap = BTreeSet::<(usize, usize)>::new();
    let mut heap2 = BTreeSet::<(usize, usize)>::new();

    for &(x, y) in &XY {
        heap.insert((x, y));
    }
    let mut ans = 0;
    for i in 0..N {
        for j in i + 1..N {
            for k in j + 1..N {
                if XY[i].0 == XY[j].0 && XY[i].1 == XY[k].1 {
                    if heap.get(&(XY[k].0, XY[j].1)).is_some() {
                        ans += 1;
                    }
                }
            }
        }
    }
    println!("{}", ans);
}
