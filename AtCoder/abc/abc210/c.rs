#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,K:usize,
        C:[usize;N],
    }
    let mut ans = 0;
    let mut mp = HashMap::<usize, usize>::new();
    for i in 0..K {
        let c = C[i];
        if !mp.contains_key(&c) {
            ans += 1;
            mp.insert(c, 1);
        } else {
            *mp.get_mut(&c).unwrap() += 1;
        }
    }
    for i in K..N {
        let mut inc = 0;
        let cr = C[i];
        let cl = C[i - K];
        if !mp.contains_key(&cr) {
            inc += 1;
            mp.insert(cr, 1);
        } else {
            *mp.get_mut(&cr).unwrap() += 1;
        }
        if *mp.get(&cl).unwrap() == 1 {
            mp.remove(&cl);
            inc -= 1;
        } else {
            *mp.get_mut(&cl).unwrap() -= 1;
        }
        let tmp = mp.len();
        ans = ans.max(tmp);
    }
    println!("{}", ans);
}
