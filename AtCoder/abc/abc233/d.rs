#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

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
        N:usize,K:i64,
        mut A:[i64;N]
    }
    A.reverse();
    A.push(0);
    A.reverse();
    A = A
        .iter()
        .scan(0, |cum, x| {
            *cum += x;
            Some(*cum)
        })
        .collect::<_>();
    let mut mp = HashMap::new();
    for i in 0..A.len() {
        let a = A[i];
        mp.entry(a).or_insert(VecDeque::new()).push_back(i);
    }
    let mut ans = 0;
    for i in 0..A.len() {
        let a = A[i];
        let b = K + a;

        if let Some(entry) = mp.get_mut(&b) {
            while entry.len() > 0 {
                let elem = entry.pop_front().unwrap();
                if elem > i {
                    entry.push_front(elem);
                    // println!("{} {} {}", i, elem, entry.len());
                    ans += entry.len();
                    break;
                }
            }
        }
    }
    println!("{}", ans);
}
