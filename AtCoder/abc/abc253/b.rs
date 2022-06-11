#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::{char,i32,f32,f64, i64, usize};

const inf_i: i64 = i64::MAX / 10;
const inf_u: usize = usize::MAX / 10;
const md1000000007: i64 = 1_000_000_007;
const md998244353: i64 = 998244353;
const eps: f64 = 1e-10;
const dy: [i64; 4] = [1, -1, 1, -1];
const dx: [i64; 4] = [1, 1, -1, -1];

fn main() {
    input! {
        H:usize,W:usize,
        maze:[Chars;H]
    }
    let mut memo = vec![];
    for h in 0..H {
        for w in 0..W {
            if maze[h][w] == 'o' {
                memo.push((h,w));
            }
        }
    }
    let mut visited = vec![vec![inf_u;W];H];
    let mut que = VecDeque::new();
    que.push_back((memo[0].0,memo[0].1, 0));
    while let Some((h,w,cnt)) = que.pop_front() {
        if visited[h][w] <= cnt {
            continue;
        }
        visited[h][w] = cnt;
        for (dh,dw) in [(-1,0),(1,0),(0,-1),(0,1)].iter() {
            let ny =  h as i64 + dh;
            let nx =  w as i64 + dw ;
            if ny<0 || H as i64<=ny ||nx<0 || W as i64<=nx {
                continue;
            }
            let ny = ny as usize;
            let nx = nx as usize;
            if visited[ny][nx] > cnt +1 {
                que.push_back((ny,nx,cnt+1));
            }

        }
    }
    println!("{}",visited[memo[1].0][memo[1].1]);

}
