#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,
        AB:[(usize,usize);N],
    }
    let mut vec = vec![];
    vec.push(0);
    for &(a, b) in &AB {
        vec.push(a);
        vec.push(a + b);
    }
    vec.sort();
    vec.dedup();
    let mut cum = vec![(0i64, 0i64); vec.len() + 1];
    let mut mp = HashMap::<i64, i64>::new();
    for &(a, b) in &AB {
        let ai = vec
            .binary_search_by_key(&(2 * a), |&v| 2 * v + 1)
            .err()
            .unwrap();
        let bi = vec
            .binary_search_by_key(&(2 * (a + b)), |&v| 2 * v + 1)
            .err()
            .unwrap();
        cum[ai] = (cum[ai].0 + 1, a as i64);
        cum[bi] = (cum[bi].0 - 1, (a + b) as i64);
        // mp.insert(ai as i64, a as i64);
        // mp.insert(bi as i64, (a + b) as i64);
    }

    for i in 1..cum.len() - 1 {
        cum[i].0 += cum[i - 1].0;
    }
    // println!("{:?}", cum);
    let mut ans = vec![0i64; N + 1];
    for i in 0..cum.len() - 1 {
        let participants = cum[i].0;
        let days = cum[i + 1].1 - cum[i].1;
        ans[participants as usize] += days as i64;
    }
    // println!("{:?}", ans);
    print!("{}", ans[1]);
    for i in 2..=N {
        print!(" {}", ans[i]);
    }
    println!("");
}
