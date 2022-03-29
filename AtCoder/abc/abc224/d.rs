#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(unused_variables)]

use proconio::marker::*;
use proconio::*;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::{f64, i64, usize};

const inf: i64 = i64::MAX / 10;
const md: i64 = 1_000_000_007;
const eps: f64 = 1e-10;

fn main() {
    input! {
        N:usize,
        UV:[(Usize1,Usize1);N],
        P:[Usize1;8],
    }
    let mut edges = vec![vec![]; 9];
    for &(u, v) in &UV {
        edges[u].push(v);
        edges[v].push(u);
    }
    let mut state = vec![inf as usize; 9];
    let mut s = inf as usize;
    for i in 0..8 {
        state[P[i]] = i + 1;
    }
    for i in 0..9 {
        if state[i] == inf as usize {
            s = i;
        }
    }

    let mut mp = HashMap::<Vec<usize>, usize>::new();
    mp.insert(state.clone(), 0);
    let mut que = VecDeque::<(usize, Vec<usize>)>::new();
    que.push_back((s, state));
    while let Some((s, state)) = que.pop_front() {
        for &nxt in &edges[s] {
            let mut nxt_state = state.clone();
            nxt_state.swap(s, nxt);
            if !mp.contains_key(&nxt_state) {
                mp.insert(nxt_state.clone(), mp.get(&state).unwrap() + 1);
                que.push_back((nxt, nxt_state));
            }
        }
    }
    // println!("{:?}", mp);
    if let Some(ans) = mp.get(&vec![1, 2, 3, 4, 5, 6, 7, 8, inf as usize]) {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
