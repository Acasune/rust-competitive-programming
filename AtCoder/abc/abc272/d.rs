#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
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
        N:usize,
        M:i64,
    }
    let mut board = vec![vec![inf_i; N]; N];
    board[0][0] = 0;

    let mut d = vec![];

    for x in 0..=M {
        let mut l = 0;
        let mut r = 1_000_100;
        while r - l > 1 {
            let m = (r + l) / 2;
            if x * x + m * m <= M {
                l = m;
            } else {
                r = m;
            }
        }
        if x * x + l * l == M {
            d.push((x, l));
            d.push((-x, l));
            d.push((x, -l));
            d.push((-x, -l));
        }
    }

    let mut que = VecDeque::<(usize, usize, i64)>::new();
    que.push_back((0, 0, 0));
    while let Some((i, j, k)) = que.pop_front() {
        for &(di, dj) in &d {
            let ni = i as i64 + di;
            let nj = j as i64 + dj;
            if 0 <= ni && ni < N as i64 && 0 <= nj && nj < N as i64 {
                let ni = ni as usize;
                let nj = nj as usize;
                if board[ni][nj] > k + 1 {
                    board[ni][nj] = k + 1;
                    que.push_back((ni, nj, k + 1));
                }
            }
        }
    }
    for i in 0..N {
        for j in 0..N {
            if board[i][j] == inf_i {
                board[i][j] = -1;
            }
        }
    }
    for b in board {
        print!("{}", b[0]);
        for c in 1..N {
            print!(" {}", b[c]);
        }
        println!();
    }
}
