#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{f64, i128, usize};

const inf: i128 = i128::MAX / 10;
const md: i128 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,
        XY:[(i128,i128);N],
    }
    let mut atans = vec![];
    for (x, y) in XY {
        let x1: i128 = x;
        let y1: i128 = y - 1;
        let x2: i128 = x - 1;
        let y2: i128 = y;
        if y1 * x2 >= y2 * x1 {
            atans.push(((y1, x1), (y2, x2)));
        } else {
            atans.push(((y2, x2), (y1, x1)));
        }
    }
    atans.sort_by(|a, b| {
        let (y1, x1) = a.0;
        let (y2, x2) = b.0;
        if y1 * x2 != y2 * x1 {
            (y1 * x2).cmp(&(y2 * x1))
        } else {
            let (y1, x1) = a.1;
            let (y2, x2) = b.1;
            (y1 * x2).cmp(&(y2 * x1))
        }
    });
    let mut upper = (0i128, 1_000_000_000_000i128);
    let mut ans = 0i128;
    for i in 0..N {
        let ((y1, x1), (y2, x2)) = atans[i];
        let (y3, x3) = upper;

        if y2 * x3 >= y3 * x2 {
            upper = (y1, x1);
            ans += 1;
        }
    }
    println!("{}", ans);
}
