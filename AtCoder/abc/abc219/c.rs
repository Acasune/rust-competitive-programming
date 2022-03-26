#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Ordering::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        X:Bytes,
        N:usize,
        mut S:[Bytes;N],
    }
    let mut mp = vec![0; 26];
    for i in 0..26 {
        for j in 0..26 {
            let x = X[j] - 'a' as u8;
            if x == i {
                mp[i as usize] = j;
            }
        }
    }
    S.sort_by(|a, b| {
        let an = a.len();
        let bn = b.len();
        for i in 0..an.min(bn) {
            let ca = (a[i] - 'a' as u8) as usize;
            let cb = (b[i] - 'a' as u8) as usize;
            if mp[ca as usize] < mp[cb as usize] {
                return Less;
            } else if mp[ca as usize] > mp[cb as usize] {
                return Greater;
            }
        }
        if an < bn {
            return Less;
        } else if an == bn {
            return Equal;
        } else {
            return Greater;
        }
    });
    for s in S {
        println!(
            "{}",
            s.into_iter().map(|i| { i as char }).collect::<String>()
        );
    }
}
