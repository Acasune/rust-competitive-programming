#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet,VecDeque};
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
        N:usize,
        xyp:[(i64,i64,i64);N]
    }
    let mut path = vec![vec![];N];
    for i in 0..N {
        for j in 0..N {
            if i == j {
                continue;
            }
            let (mut x0, mut y0, p) = xyp[i];
            let (mut x1,mut y1,_) = xyp[j];
            path[i].push((((i64::abs(x0-x1) + i64::abs(y0-y1))+p-1)/p,j))
        }
    }
    // println!("{:?}",path);
    for i in 0..N {
        path[i].sort();
    }
    let mut ng = 0;
    let mut ok = inf_i;
    while ok > ng +1 {
        let m = (ok +ng)/2;
        // println!("{}",m);
        let mut flg =false;
        for i in 0..N {
            flg |=can_reach(&path,m,i);
        }
        if flg {
            ok = m;
        } else {
            ng =m;
        }
    }
    println!("{}",ok-1);
}

fn can_reach(path:&Vec<Vec<(i64,usize)>>,m:i64,s:usize) -> bool{
    let mut visited = vec![false;path.len()];
    let mut que = VecDeque::new();
    visited[s] = true;
    que.push_back(s);
    while let Some(s) = que.pop_front() {
        for i in 0..path[s].len() {
            let (q,j) = path[s][i];
            if visited[j] {
                continue;
            }
            if m < q {
                break;
            }
            visited[j] = true;
            que.push_back(j);
        }
    }
    let ret = visited.into_iter().filter(|a| !a).count() >0;
    return !ret;


}
