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

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 8] = [1, 1, 0, -1, -1, -1, 0, 1];
const dx: [i64; 8] = [0, 1, 1, 1, 0, -1, -1, -1];

fn main() {
    input! {
        N:usize,
        B:[Chars;N],
    }
    let mut A = vec![];
    for b in B {
        A.push(
            b.into_iter()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect(),
        );
    }
    let mut ans = 0;
    for i in 0..N {
        for j in 0..N {
            for (&y, &x) in dy.iter().zip(dx.iter()) {
                let tmp = calc(i, j, y, x, &A);
                ans = ans.max(tmp);
            }
        }
    }
    println!("{}", ans);
}

fn calc(sy: usize, sx: usize, dir_y: i64, dir_x: i64, map: &Vec<Vec<usize>>) -> usize {
    let mut y = sy;
    let mut x = sx;
    let mut ret = 0;
    let n = map.len();
    for k in 0..n {
        ret *= 10;
        ret += map[y][x];
        y = (((n + y) as i64 + dir_y) % n as i64) as usize;
        x = (((n + x) as i64 + dir_x) % n as i64) as usize;
    }
    return ret;
}
