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
        N:usize,mut K:i64,
        A:[i64;N],
    }
    let mut mp = HashMap::<i64, i64>::new();
    for a in A {
        *mp.entry(a).or_insert(0) += 1;
    }

    let mut heap = BinaryHeap::<(i64, i64)>::new();
    for e in mp.into_iter() {
        heap.push((e.0, e.1));
    }
    heap.push((0, 0));

    let mut ans = 0i64;

    while heap.len() > 1 {
        if K == 0 {
            break;
        }
        let (mut a1, n1) = heap.pop().unwrap();
        if a1 == 0 {
            break;
        }
        let (a2, n2) = heap.pop().unwrap();
        if (a1 - a2) * n1 <= K {
            ans += n1 * ((a1 + a2 + 1) * (a1 - a2) / 2);
            K -= n1 * (a1 - a2);
            heap.push((a2, n1 + n2));
        } else {
            while n1 < K && a1 > 0 {
                ans += n1 * a1;
                a1 -= 1;
                K -= n1;
            }
            ans += a1 * K;
            break;
        }
    }
    println!("{}", ans);
}
