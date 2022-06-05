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
        N:usize,M:usize,
        AB:[(Usize1, Usize1);M],
        Q:usize,
        XK:[(Usize1,usize);Q],
    }
    let mut edges = vec![vec![];N];
    for &(a,b) in &AB {
        edges[a].push(b);
        edges[b].push(a);
    }
    for &(x,k) in &XK {
        
        let mut deq = VecDeque::new();
        let mut visited = vec![false;N];
        visited[x] =true;
        let mut ans = x+1;
        deq.push_back((x,0));
        while let Some((s,sum)) = deq.pop_front() {
            for &t in &edges[s] {
                if !visited[t] && sum+1<=k{
                    visited[t] =true;
                    ans+=t+1;
                    deq.push_back((t,sum+1));
                }
            }
        }
        println!("{}",ans);
    }

}
