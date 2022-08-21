#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::convert::TryInto;
use std::{char, f32, f64, i32, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,
        mut AX:i64,mut AY:i64,
        mut BX:i64, mut BY:i64,
        board:[Chars;N]
    }
    AX -= 1;
    AY -= 1;
    BX -= 1;
    BY -= 1;
    let mut visited = vec![vec![vec![inf_i; N]; N]; 4];

    let mut que = VecDeque::new();
    for i in 0..4 {
        que.push_back((AX, AY, i));
        visited[i][AX as usize][AY as usize] = 1;
    }
    while let Some((ax, ay, d)) = que.pop_front() {
        for (i, (dx, dy)) in [(1, 1), (1, -1), (-1, 1), (-1, -1)].iter().enumerate() {
            if 0 <= ax + dx && ax + dx < N as i64 && 0 <= ay + dy && ay + dy < N as i64 {
                let nx = (ax + dx) as usize;
                let ny = (ay + dy) as usize;
                if board[nx][ny] == '#' {
                    continue;
                }

                let mut distance = visited[d as usize][ax as usize][ay as usize] + 1;
                if i == d {
                    distance -= 1;
                }
                if visited[i][nx][ny] > distance {
                    visited[i][nx][ny] = distance;
                    que.push_back((nx as i64, ny as i64, i));
                }
            }
        }
    }
    let mut ans = inf_i;

    for i in 0..4 {
        ans = ans.min(visited[i][BX as usize][BY as usize]);
    }
    if ans == inf_i {
        println!("{}", -1);
    } else {
        println!("{}", ans);
    }
}
