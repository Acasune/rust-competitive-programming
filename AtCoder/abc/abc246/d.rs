#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
use std::{f64, i128, usize};

const inf: i128 = i128::MAX / 10;
const md: i128 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:i128,
    }
    let mut a = 0;
    let mut b = 0;
    let mut vec = vec![];
    loop {
        if a * a * a > N {
            break;
        }
        let mut r = 1_000_000_000;
        // println!("{}", r);
        let mut l = -1;
        let mut m = 0;
        while r - l > 1 {
            m = (r - l) / 2 + l;
            let ret = ret(a, m);
            if ret >= N {
                r = m;
            } else {
                l = m;
            }
            // println!("{} {}", l, r);
        }
        let mut tmp = ret(a, r);
        if tmp >= N {
            vec.push(tmp);
        }

        a += 1;
    }

    vec.sort();

    println!("{}", vec[0]);
    // for a in 0..10 {
    //     for b in 0..10 {
    //         print!(" {}", ret(a, b));
    //     }
    //     println!("");
    // }
}

fn ret(a: i128, b: i128) -> i128 {
    (a + b) * (a * a + b * b)
}
