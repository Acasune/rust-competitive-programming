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
        H:usize,W:usize,
        maze:[Chars;H]
    }
    let mut cnt = 0i64;
    let mut h = 0;
    let mut w = 0;
    let mut visited = vec![vec![false; W]; H];
    visited[0][0] = true;
    loop {
        let mut nh = h;
        let mut nw = w;

        let d = maze[h as usize][w as usize];
        if d == 'U' {
            nh -= 1;
        } else if d == 'D' {
            nh += 1;
        } else if d == 'L' {
            nw -= 1;
        } else {
            nw += 1;
        }
        if !check(nh, nw, H, W) {
            break;
        }
        if visited[nh as usize][nw as usize] {
            println!("{}", -1);
            return;
        }
        h = nh;
        w = nw;
        visited[h as usize][w as usize] = true;
    }
    println!("{} {}", h + 1, w + 1);
}

fn check(h: i64, w: i64, H: usize, W: usize) -> bool {
    (0 <= h && h < H as i64 && 0 <= w && w < W as i64)
}
