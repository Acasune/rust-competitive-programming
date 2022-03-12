#![allow(unused_imports)]
#![allow(non_snake_case)]
use std::collections::{BinaryHeap, HashMap};

use proconio::marker::*;
use proconio::*;
use std::{f64, i64};

fn main() {
    input! {
        N:usize,
        XY:[(i64, i64);N],
        S:Chars
    }
    let inf = i64::MAX / 10;
    let mut mp1 = HashMap::<i64, i64>::new();
    let mut mp2 = HashMap::<i64, i64>::new();
    for i in 0..N {
        let dir = if S[i] == 'R' { 1 } else { -1 };
        // 1 -> R, -1 -> L
        let (x, y) = XY[i];
        if dir == 1 {
            if !mp1.contains_key(&y) || x < *mp1.get(&y).unwrap() {
                mp1.insert(y, x);
            }
        } else {
            if !mp2.contains_key(&y) || *mp2.get(&y).unwrap() < x {
                mp2.insert(y, x);
            }
        }
    }
    for i in 0..N {
        let (x, y) = XY[i];
        if !mp1.contains_key(&y) || !mp2.contains_key(&y) {
            continue;
        }

        if *mp1.get(&y).unwrap() <= *mp2.get(&y).unwrap() {
            println!("{}", "Yes");
            return;
        }
    }
    println!("{}", "No");
}
