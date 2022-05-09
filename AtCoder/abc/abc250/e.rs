#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
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
        As:[usize;N],
        Bs:[usize;N],
        Q:usize,
        XY:[(Usize1,Usize1);Q]
    }

    let mut a_set = HashSet::new();
    let mut b_set = HashSet::new();
    let mut ab_set = HashSet::new();
    let mut b_i = 0;
    let mut vec: Vec<Vec<usize>> = vec![];
    for i in 0..N {
        let a = As[i];
        if ab_set.contains(&a) {
            vec.push(vec[i - 1].clone());
        } else {
            if b_set.contains(&a) {
                ab_set.insert(a);
                b_set.remove(&a);
            } else {
                a_set.insert(a);
            }
            while b_i < N && a_set.len() != 0 && b_set.len() == 0 {
                let b = Bs[b_i];
                if ab_set.contains(&b) {
                } else if a_set.contains(&b) {
                    ab_set.insert(b);
                    a_set.remove(&b);
                } else {
                    b_set.insert(b);
                }
                b_i += 1;
            }
            let l = b_i - 1;
            if a_set.len() > 0 {
                vec.push(vec![N, N]);
            } else {
                while b_i < N && ab_set.contains(&Bs[b_i]) {
                    b_i += 1;
                }
                vec.push(vec![l, b_i]);
            }
        }
    }
    // println!("{:?}", vec);
    for (x, y) in XY {
        if vec[x][0] <= y && y < vec[x][1] {
            println!("{}", "Yes");
        } else {
            println!("{}", "No");
        }
    }
}
