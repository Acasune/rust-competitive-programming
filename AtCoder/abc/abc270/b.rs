#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{char, f32, f64, i32, i64, usize};

const inf_i: i64 = (i64::MAX / 10) * 9;
const inf_u: usize = (usize::MAX / 10) * 9;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const d8yx: [(i64, i64); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];
const d4yx: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    input! {
        X:i64,Y:i64,Z:i64
    }
    if X >= 0 {
        if Y < 0 || X < Y {
            println!("{}", X);
            return;
        } else {
            if Y < Z {
                println!("{}", -1);
                return;
            } else {
                let ans = Z.abs() + X - Z;
                println!("{}", ans);
                return;
            }
        }
    } else {
        if Y < X || 0 < Y {
            println!("{}", -X);
            return;
        } else {
            if Z < Y {
                println!("{}", -1);
                return;
            } else {
                let ans = Z.abs() + (X - Z).abs();
                println!("{}",ans);
                return;
            }
        }
    }
}