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
const d6yx: [(i64, i64); 6] = [(-1, -1), (-1, 0), (0, -1), (0, 1), (1, 0), (1, 1)];
const d4yx: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    input! {
        N:usize,
        XY:[(i64,i64);N]
    }
    let mut map = vec![vec![false; 2010]; 2010];
    for (x, y) in XY {
        map[(x + 1005) as usize][(y + 1005) as usize] = true;
    }
    let mut visited = vec![vec![0; 2010]; 2010];
    let mut cur = 1;
    let mut que = vec![];
    for j in 0..2010 {
        for i in 0..2010 {
            if map[j][i] && visited[j][i] == 0 {
                que.push((j as i64, i as i64));
                while let Some((sj, si)) = que.pop() {
                    for &(dj, di) in &d6yx {
                        let nj = sj + dj;
                        let ni = si + di;
                        if 0 <= nj && nj < 2010 && 0 <= ni && ni < 2010 {
                            let nj = nj as usize;
                            let ni = ni as usize;
                            if map[nj][ni] && visited[nj][ni] == 0 {
                                visited[nj][ni] = cur;
                                que.push((nj as i64, ni as i64));
                            }
                        }
                    }
                }
                cur += 1;
            }
        }
    }
    println!("{}", cur - 1);
}
