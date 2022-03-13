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
const eps: f64 = 0.000_000_000_001;

fn main() {
    input! {
        N:usize,
        mut AB:[(f64,f64);N],
        mut CD:[(f64,f64);N],
    }
    let mut cx = 0.0;
    let mut cy = 0.0;
    for i in 0..N {
        cx += AB[i].0;
        cy += AB[i].1;
        AB[i].0 *= N as f64;
        AB[i].1 *= N as f64;
    }
    for i in 0..N {
        AB[i].0 -= cx;
        AB[i].1 -= cy;
    }
    let mut cx = 0.0;
    let mut cy = 0.0;
    for i in 0..N {
        cx += CD[i].0;
        cy += CD[i].1;
        CD[i].0 *= N as f64;
        CD[i].1 *= N as f64;
    }
    for i in 0..N {
        CD[i].0 -= cx;
        CD[i].1 -= cy;
    }
    for i in 0..N {
        if AB[i].0 - 0.0 > eps || AB[i].1 - 0.0 > eps {
            let x = AB[i].0;
            let y = AB[i].1;
            AB[i].0 = AB[0].0;
            AB[i].1 = AB[0].1;
            AB[0].0 = x;
            AB[0].1 = y;
        }
    }
    let mut ans = "No";
    for i in 0..N {
        let angle = CD[i].1.atan2(CD[i].0) - AB[0].1.atan2(AB[0].0);
        let mut flg = true;
        for j in 0..N {
            let a = AB[j].0 * f64::cos(angle) - AB[j].1 * f64::sin(angle);
            let b = AB[j].0 * f64::sin(angle) + AB[j].1 * f64::cos(angle);
            let mut flg2 = false;
            for k in 0..N {
                if (a - CD[k].0).abs() < eps && (b - CD[k].1).abs() < eps {
                    flg2 = true;
                }
            }
            flg &= flg2;
        }
        if flg {
            ans = "Yes"
        }
    }

    println!("{}", ans);
}
