#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, BTreeSet};
use std::convert::TryInto;
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
        N:usize,u:Usize1, v:Usize1,
        AB:[(Usize1,Usize1);N-1],
    }
    let mut aoki = vec![inf_u;N];
    let mut taka = vec![inf_u;N];
    let mut edges = vec![vec![];N];
    for (a,b) in AB {
        edges[a].push(b);
        edges[b].push(a);
    }
    aoki[v] = 0;
    aoki_dfs(v, 1, &mut aoki, &edges);
    taka[u] = 0;
    taka_dfs(u, 1, &mut taka,&aoki, &edges);
    // println!("{:?}",aoki);
    // println!("{:?}",taka);
    let mut ans = 0;
    for i in 0..N {
        let a = aoki[i];
        let b = taka[i];
        if b == inf_u {
            continue;
        }
        // println!("{} {} {}",i,aoki[i], taka[i]);
        ans = ans.max(a);
    }
    println!("{}",ans-1);
}

fn aoki_dfs(s:usize,cnt:usize,aoki:&mut Vec<usize>,edges:&Vec<Vec<usize>>) {
    for &v in &edges[s] {
        if cnt < aoki[v] {
            aoki[v] = cnt;
            aoki_dfs(v, cnt+1, aoki, edges)
        }
    }
}

fn taka_dfs(s:usize,cnt:usize,taka:&mut Vec<usize>,aoki:&Vec<usize>,edges:&Vec<Vec<usize>>) {
    for &v in &edges[s] {
        if cnt < taka[v] && cnt <aoki[v] {
            taka[v] = cnt;
            taka_dfs(v, cnt+1,taka, aoki, edges)
        }
    }
}
